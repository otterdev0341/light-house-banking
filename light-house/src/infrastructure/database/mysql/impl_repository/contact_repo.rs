use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, QueryTrait, TransactionTrait};
use uuid::Uuid;

use crate::{domain::{dto::contact_dto::{ReqCreateContactDto, ReqUpdateContactDto}, entities::{contact, user_contact}, req_repository::{contact_repository::{ContactRepositoryBase, ContactRepositoryUtility}, contact_type_repository::ContactTypeRepositoryBase}}, soc::soc_repository::RepositoryError};





pub struct ContactRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}

impl ContactRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait]
impl ContactRepositoryBase for ContactRepositoryImpl{
    
    async fn create(
        &self, 
        user_id: Uuid, 
        dto: ReqCreateContactDto
    ) 
        -> Result<contact::Model, RepositoryError>
    {
        // Execute the transaction
        let result = self.db_pool.transaction::<_, contact::Model, RepositoryError>(|txn| {
            Box::pin(async move {
                // Create the ActiveModel for the contact
                let new_contact = contact::ActiveModel {
                    id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the contact
                    name: Set(dto.name),
                    business_name: Set(dto.business_name),
                    phone: Set(dto.phone),
                    description: Set(dto.description),
                    contact_type_id: Set(dto.contact_type_id.as_bytes().to_vec()), // Set the contact type ID
                    ..Default::default()
                };

                // Insert the contact into the database
                let inserted_contact = new_contact
                    .insert(txn)
                    .await
                    .map_err(|err| {
                        if let sea_orm::DbErr::Exec(exec_err) = &err {
                            if exec_err.to_string().contains("UNIQUE") {
                                return RepositoryError::UniqueConstraintViolation(
                                    "Contact with the same phone or business name already exists"
                                        .to_string(),
                                );
                            }
                        }
                        RepositoryError::DatabaseError(err.to_string())
                    })?;

                // Create the ActiveModel for the user_contact relationship
                let new_user_contact = user_contact::ActiveModel {
                    user_id: Set(user_id.as_bytes().to_vec()),
                    contact_id: Set(inserted_contact.id.clone()), // Use the ID of the newly created contact
                    ..Default::default()
                };

                // Insert the user_contact relationship into the database
                new_user_contact
                    .insert(txn)
                    .await
                    .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

                // Return the inserted contact
                Ok(inserted_contact)
            })
        })
        .await;

        // Return the result of the transaction
        match result {
            Ok(contact) => Ok(contact),
            Err(err) => Err(RepositoryError::OperationFailed(err.to_string())),
        }
    }


    async fn find_by_id(
        &self, 
        contact_id: Uuid
    ) 
        -> Result<Option<contact::Model>, RepositoryError>
    {
        // Query the database to find the contact by ID and ensure it belongs to the user
        let contact = contact::Entity::find()
            .filter(contact::Column::Id.eq(contact_id.as_bytes().to_vec())) // Filter by contact ID
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the contact if found, or None if not found
        Ok(contact)
    }


    async fn find_all(
        &self
    ) 
        -> Result<Vec<contact::Model>, RepositoryError>
    {
        // Query the database to retrieve all contacts
        let contacts = contact::Entity::find()
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of contacts
        Ok(contacts)
        
    }


    async fn update(
        &self, 
        dto: ReqUpdateContactDto, 
        user_id: Uuid, 
        contact_id: Uuid
    ) 
        -> Result<contact::Model, RepositoryError>
    {
        // Ensure the contact belongs to the user by checking the user_contact relationship
        let contact_exists = user_contact::Entity::find()
            .filter(user_contact::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .filter(user_contact::Column::ContactId.eq(contact_id.as_bytes().to_vec())) // Filter by contact ID
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if contact_exists.is_none() {
            return Err(RepositoryError::NotFound(format!(
                "Contact with ID {} not found for user {}",
                contact_id, user_id
            )));
        }

        // Find the contact to update
        let contact = contact::Entity::find()
            .filter(contact::Column::Id.eq(contact_id.as_bytes().to_vec())) // Filter by contact ID
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        let contact = match contact {
            Some(contact) => contact,
            None => {
                return Err(RepositoryError::NotFound(format!(
                    "Contact with ID {} not found",
                    contact_id
                )))
            }
        };

        // Convert the found contact into an ActiveModel for updating
        let mut active_model: contact::ActiveModel = contact.into();

        // Update fields if they are provided in the DTO
        if let Some(name) = dto.name {
            if !name.is_empty() {
                active_model.name = Set(name);
            }
        }
        if let Some(business_name) = dto.business_name {
            if !business_name.is_empty() {
                active_model.business_name = Set(business_name);
            }
        }
        if let Some(phone) = dto.phone {
            if !phone.is_empty() {
                active_model.phone = Set(phone);
            }
        }
        if let Some(description) = dto.description {
            if !description.is_empty() {
                active_model.description = Set(description);
            }
        }
        if let Some(contact_type_id) = dto.contact_type_id {
            active_model.contact_type_id = Set(contact_type_id.as_bytes().to_vec());
        }


        // Save the updated contact to the database
        let updated_contact = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Contact with the same phone or business name already exists"
                                .to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(updated_contact)
    }


    async fn delete(
        &self,
        user_id: Uuid, 
        contact_id : Uuid
    ) 
        -> Result<(), RepositoryError>
    {
        // Execute the transaction
        let result = self.db_pool.transaction::<_, (), RepositoryError>(|txn| {
            Box::pin(async move {
                // Ensure the contact belongs to the user by checking the user_contact relationship
                let contact_exists = user_contact::Entity::find()
                    .filter(user_contact::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
                    .filter(user_contact::Column::ContactId.eq(contact_id.as_bytes().to_vec())) // Filter by contact ID
                    .one(txn)
                    .await
                    .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

                if contact_exists.is_none() {
                    return Err(RepositoryError::NotFound(format!(
                        "Contact with ID {} not found for user {}",
                        contact_id, user_id
                    )));
                }

                // Delete the user_contact relationship first to avoid foreign key constraint violations
                user_contact::Entity::delete_many()
                    .filter(user_contact::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
                    .filter(user_contact::Column::ContactId.eq(contact_id.as_bytes().to_vec())) // Filter by contact ID
                    .exec(txn)
                    .await
                    .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

                // Delete the contact
                contact::Entity::delete_many()
                    .filter(contact::Column::Id.eq(contact_id.as_bytes().to_vec())) // Filter by contact ID
                    .exec(txn)
                    .await
                    .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

                Ok(())
            })
        })
        .await;

        // Match the result of the transaction and return the appropriate value or error
        match result {
            Ok(_) => Ok(()),
            Err(sea_orm::TransactionError::Transaction(err)) => match err {
                RepositoryError::NotFound(msg) => Err(RepositoryError::NotFound(msg)),
                RepositoryError::DatabaseError(msg) => Err(RepositoryError::DatabaseError(msg)),
                other => Err(RepositoryError::OperationFailed(other.to_string())),
            },
            Err(err) => Err(RepositoryError::OperationFailed(err.to_string())),
        }
    }
}

#[async_trait::async_trait]
impl ContactRepositoryUtility for ContactRepositoryImpl{


    async fn find_all_by_user_id(
        &self, 
        user_id: Uuid
    ) 
        -> Result<Vec<contact::Model>, RepositoryError>
    {
         // Query the database to retrieve all contacts for the given user
         let contacts = contact::Entity::find()
         .filter(
             contact::Column::Id.in_subquery(
                 user_contact::Entity::find()
                     .select_only()
                     .column(user_contact::Column::ContactId)
                     .filter(user_contact::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
                     .into_query(),
             ),
         )
         .all(self.db_pool.as_ref())
         .await
         .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

     // Return the list of contacts
     Ok(contacts)
    }


    async fn find_by_user_id_and_contact_type_id(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) 
        -> Result<Option<contact::Model>, RepositoryError>
    {
        // Query the database to find a contact by user_id and contact_type_id
        let contact = contact::Entity::find()
            .filter(
                contact::Column::Id.in_subquery(
                    user_contact::Entity::find()
                        .select_only()
                        .column(user_contact::Column::ContactId)
                        .filter(user_contact::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
                        .into_query(),
                ),
            )
            .filter(contact::Column::ContactTypeId.eq(contact_id.as_bytes().to_vec())) // Filter by contact type ID
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the contact if found, or None if not found
        Ok(contact)
    }


    async fn is_in_use_in_transaction(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) 
        -> Result<bool, RepositoryError>
    {
         // Query the database to check if the contact is in use in any transaction
         let is_in_use = user_contact::Entity::find()
         .filter(user_contact::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
         .filter(user_contact::Column::ContactId.eq(contact_id.as_bytes().to_vec())) // Filter by contact ID
         .count(self.db_pool.as_ref()) // Count the records
         .await
         .map(|count| count > 0) // Check if count is greater than zero
         .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return true if the contact is in use, otherwise false
        Ok(is_in_use)
    }

    async fn find_by_user_id_and_contact_id(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) 
        -> Result<Option<contact::Model>, RepositoryError>
    {
        // Query the database to find a contact by user_id and contact_id
        let contact = contact::Entity::find()
            .filter(
                contact::Column::Id.in_subquery(
                    user_contact::Entity::find()
                        .select_only()
                        .column(user_contact::Column::ContactId)
                        .filter(user_contact::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
                        .filter(user_contact::Column::ContactId.eq(contact_id.as_bytes().to_vec())) // Filter by contact ID
                        .into_query(),
                ),
            )
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the contact if found, or None if not found
        Ok(contact)
    }
}