use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{dto::expense_type_dto::{ReqCreateExpenseTypeDto, ReqUpdateExpenseTypeDto}, entities::{expense, expense_type}, req_repository::expense_type_repository::{ExpenseTypeRepositoryBase, ExpenseTypeRepositoryUtility}}, soc::soc_repository::RepositoryError};





pub struct ExpenseTypeRepositoryImpl{
    db_pool: Arc<DatabaseConnection>
}

impl ExpenseTypeRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait]
impl ExpenseTypeRepositoryBase for ExpenseTypeRepositoryImpl {
    async fn create(&self, user_id: Uuid, dto: ReqCreateExpenseTypeDto) 
        -> Result<expense_type::Model, RepositoryError>
    {
        // Create the ActiveModel for the expense type
        let new_expense_type = expense_type::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the expense type
            name: Set(dto.name),                        // Set the name from the DTO
            user_id: Set(user_id.as_bytes().to_vec()),  // Associate the expense type with the user
            ..Default::default()
        };

        // Insert the expense type into the database
        let inserted_expense_type = new_expense_type
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Expense type with the same name already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(inserted_expense_type)
    }


    async fn find_by_id(&self, expesne_type_id: Uuid) 
        -> Result<Option<expense_type::Model>, RepositoryError>
    {
        // Query the database to find the expense type by ID and ensure it belongs to the user
        let expense_type = expense_type::Entity::find()
            .filter(expense_type::Column::Id.eq(expesne_type_id.as_bytes().to_vec())) // Filter by expense type ID
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the expense type if found, or None if not found
        Ok(expense_type)
    }


    async fn find_all(&self) 
        -> Result<Vec<expense_type::Model>, RepositoryError>
    {
        // Query the database to retrieve all expense types
        let expense_types = expense_type::Entity::find()
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of expense types
        Ok(expense_types)
    }


    async fn update(&self, dto: ReqUpdateExpenseTypeDto, user_id: Uuid, expense_type_id: Uuid) 
    -> Result<expense_type::Model, RepositoryError>
    {
        // Ensure the expense type belongs to the user
        let expense_type_exists = expense_type::Entity::find()
            .filter(expense_type::Column::Id.eq(expense_type_id.as_bytes().to_vec())) // Filter by expense type ID
            .filter(expense_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // If the expense type does not exist, return a NotFound error
        let existing_expense_type = match expense_type_exists {
            Some(expense_type) => expense_type,
            None => {
                return Err(RepositoryError::NotFound(format!(
                    "Expense type with ID {} not found for user {}",
                    expense_type_id, user_id
                )));
            }
        };

        // Create the ActiveModel for the update
        let mut active_model: expense_type::ActiveModel = existing_expense_type.into();

        // Update fields if they are provided in the DTO
        if let Some(name) = dto.name {
            active_model.name = Set(name);
        }
        

        // Save the updated expense type to the database
        let updated_expense_type = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Expense type with the same name already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(updated_expense_type)
    }


    async fn delete(&self,user_id: Uuid, expense_type_id : Uuid) 
    -> Result<(), RepositoryError>
    {
        // Check if the expense type is in use
        let is_in_use = self
            .is_in_use(user_id, expense_type_id)
            .await
            .map_err(|err| RepositoryError::NotFound(err.to_string()))?;

        if is_in_use {
            return Err(RepositoryError::OperationFailed(format!(
                "Expense type with ID {} is currently in use and cannot be deleted",
                expense_type_id
            )));
        }

        // Ensure the expense type belongs to the user
        let expense_type_exists = expense_type::Entity::find()
            .filter(expense_type::Column::Id.eq(expense_type_id.as_bytes().to_vec())) // Filter by expense type ID
            .filter(expense_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if expense_type_exists.is_none() {
            return Err(RepositoryError::NotFound(format!(
                "Expense type with ID {} not found for user {}",
                expense_type_id, user_id
            )));
        }

        // Delete the expense type
        expense_type::Entity::delete_many()
            .filter(expense_type::Column::Id.eq(expense_type_id.as_bytes().to_vec())) // Filter by expense type ID
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(())
    }
}




#[async_trait::async_trait]
impl ExpenseTypeRepositoryUtility for ExpenseTypeRepositoryImpl{
    async fn is_in_use(&self, user_id: Uuid, expense_type_id: Uuid) 
    -> Result<bool, RepositoryError>
    {
        // Query the database to check if the expense type is in use
        let is_in_use = expense::Entity::find()
            .filter(expense::Column::ExpenseTypeId.eq(expense_type_id.as_bytes().to_vec())) // Filter by expense type ID
            .filter(expense::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .count(self.db_pool.as_ref()) // Check if any record exists
            .await
            .map_err(|err| RepositoryError::from(err))?;

        // Return true if the expense type is in use, otherwise false
        Ok(is_in_use > 0)
    }


    async fn find_all_by_user_id(&self, user_id: Uuid) 
    -> Result<Vec<expense_type::Model>, RepositoryError>
    {
        // Query the database to retrieve all expense types for the given user
        let expense_types = expense_type::Entity::find()
            .filter(expense_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;

        // Return the list of expense types
        Ok(expense_types)
    }

    async fn find_by_user_id_and_expense_type_id(&self, expense_type_id: Uuid, user_id: Uuid) 
    -> Result<Option<expense_type::Model>, RepositoryError>
    {
        // Query the database to find the expense type by ID and ensure it belongs to the user
        let expense_type = expense_type::Entity::find()
            .filter(expense_type::Column::Id.eq(expense_type_id.as_bytes().to_vec())) // Filter by expense type ID
            .filter(expense_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the expense type if found, or None if not found
        Ok(expense_type)
    }
}