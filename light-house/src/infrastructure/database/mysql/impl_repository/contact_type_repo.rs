use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{dto::contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto}, entities::contact_type, req_repository::contact_type_repository::{ContactTypeRepositoryBase, ContactTypeRepositoryUtility}}, soc::soc_repository::RepositoryError};






pub struct ContactTypeRepositoryImpl{
    pub db_pool: Arc<DatabaseConnection>
}

impl ContactTypeRepositoryImpl{
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait]
impl ContactTypeRepositoryBase for ContactTypeRepositoryImpl{
    async fn create(
        &self, 
        user_id: Uuid, 
        dto: ReqCreateContactTypeDto
    ) 
        -> Result<contact_type::Model, RepositoryError>
    {
        // Create the ActiveModel for the contact type
        let new_contact_type = contact_type::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the contact type
            name: Set(dto.name),
            user_id: Set(user_id.as_bytes().to_vec()), // Set the user ID
            ..Default::default()
        };

        // Insert the contact type into the database
        let inserted_contact_type = new_contact_type
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Contact type name already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(inserted_contact_type)
    }


    async fn find_by_id(
        &self, 
        user_id: Uuid, 
        contact_type_id: Uuid
    ) 
        -> Result<Option<contact_type::Model>, RepositoryError>
    {
         // Query the database to find the contact type by ID and ensure it belongs to the user
         let contact_type = contact_type::Entity::find()
         .filter(contact_type::Column::Id.eq(contact_type_id.as_bytes().to_vec())) // Filter by contact type ID
         .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
         .one(self.db_pool.as_ref())
         .await
         .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

     // Return the contact type if found, or None if not found
     Ok(contact_type)
    }


    async fn find_all(
        &self
    ) 
        -> Result<Vec<contact_type::Model>, RepositoryError>
    {
         // Query the database to retrieve all contact types
         let contact_types = contact_type::Entity::find()
         .all(self.db_pool.as_ref())
         .await
         .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of contact types
        Ok(contact_types)
    }


    async fn update(
        &self, dto: ReqUpdateContactTypeDto, 
        user_id: Uuid, 
        contact_type_id: Uuid
    ) 
        -> Result<contact_type::Model, RepositoryError>
    {
        // Find the contact type by ID and ensure it belongs to the user
        let contact_type = contact_type::Entity::find()
            .filter(contact_type::Column::Id.eq(contact_type_id.as_bytes().to_vec())) // Filter by contact type ID
            .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return an error if the contact type is not found
        let contact_type = match contact_type {
            Some(contact_type) => contact_type,
            None => {
                return Err(RepositoryError::NotFound(format!(
                    "Contact type with ID {} not found for user {}",
                    contact_type_id, user_id
                )))
            }
        };

        // Convert the found contact type into an ActiveModel for updating
        let mut active_model: contact_type::ActiveModel = contact_type.into();

        // Update fields if they are provided in the DTO
        if let Some(name) = dto.name {
            if !name.is_empty() {
                active_model.name = Set(name);
            }
        }
        

        // Save the updated contact type to the database
        let updated_contact_type = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Contact type name already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(updated_contact_type)
    }


    async fn delete(
        &self,
        user_id: Uuid, 
        contact_type_id : Uuid
    ) 
        -> Result<(), RepositoryError>
    {
        // Attempt to delete the contact type by ID and ensure it belongs to the user
        let result = contact_type::Entity::delete_many()
            .filter(contact_type::Column::Id.eq(contact_type_id.as_bytes().to_vec())) // Filter by contact type ID
            .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Check if any rows were affected (i.e., if the contact type was deleted)
        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound(format!(
                "Contact type with ID {} not found for user {}",
                contact_type_id, user_id
            )));
        }

        Ok(())
    }
}


#[async_trait::async_trait]
impl ContactTypeRepositoryUtility for ContactTypeRepositoryImpl {
    
    async fn find_by_name(
        &self, 
        name: &str,
        user_id: Uuid
    ) 
        -> Result<Option<contact_type::Model>, RepositoryError>
    {
        // Query the database to find the contact type by name and ensure it belongs to the user
        let contact_type = contact_type::Entity::find()
            .filter(contact_type::Column::Name.eq(name)) // Filter by contact type name
            .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;

        // Return the contact type if found, or None if not found
        Ok(contact_type)
    }


    async fn find_all_by_user_id(
        &self, 
        user_id: Uuid
    ) 
        -> Result<Vec<contact_type::Model>, RepositoryError>
    {
         // Query the database to retrieve all contact types for the given user
         let contact_types = contact_type::Entity::find()
         .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
         .all(self.db_pool.as_ref())
         .await
         .map_err(|err| RepositoryError::from(err))?;

     // Return the list of contact types
        Ok(contact_types)
    }
}