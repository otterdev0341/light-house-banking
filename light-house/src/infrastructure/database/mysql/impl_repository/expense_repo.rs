use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{dto::expense_dto::{ReqCreateExpenseDto, ReqUpdateExpenseDto}, entities::{expense, transaction}, req_repository::expense_repository::{ExpenseRepositoryBase, ExpenseRepositoryUtill}}, soc::soc_repository::RepositoryError};





pub struct ExpenseRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}

impl ExpenseRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self {
            db_pool
        }
    }
}

#[async_trait::async_trait]
impl ExpenseRepositoryBase for ExpenseRepositoryImpl{
    async fn create(&self, user_id: Uuid, dto: ReqCreateExpenseDto) 
    -> Result<expense::Model, RepositoryError>
    {
        // Create the ActiveModel for the expense
        let new_expense = expense::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the expense
            description: Set(dto.description),          // Set the description from the DTO
            expense_type_id: Set(dto.expense_type_id.as_bytes().to_vec()),  // Set the expense type ID from the DTO
            user_id: Set(user_id.as_bytes().to_vec()),  // Associate the expense with the user
            ..Default::default()
        };

        // Insert the expense into the database
        let inserted_expense = new_expense
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("FOREIGN KEY") {
                        return RepositoryError::OperationFailed(
                            "Invalid expense type ID or user ID".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(inserted_expense)
    }

    async fn find_by_id(&self, expense_id: Uuid) 
    -> Result<Option<expense::Model>, RepositoryError> 
    {
        // Query the database to find the expense by ID
        let expense = expense::Entity::find()
            .filter(expense::Column::Id.eq(expense_id.as_bytes().to_vec())) // Filter by expense ID
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the expense if found, or None if not found
        Ok(expense)
    }

    async fn find_all(&self) 
    -> Result<Vec<expense::Model>, RepositoryError>
    {
        // Query the database to retrieve all expenses
        let expenses = expense::Entity::find()
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of expenses
        Ok(expenses)
    }

    async fn update(&self, dto: ReqUpdateExpenseDto, user_id: Uuid, expense_id: Uuid) 
    -> Result<expense::Model, RepositoryError>
    {
        // Ensure the expense belongs to the user
        let expense_exists = expense::Entity::find()
            .filter(expense::Column::Id.eq(expense_id.as_bytes().to_vec())) // Filter by expense ID
            .filter(expense::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if expense_exists.is_none() {
            return Err(RepositoryError::NotFound(format!(
                "Expense with ID {} not found for user {}",
                expense_id, user_id
            )));
        }

        // Convert the existing expense into an ActiveModel for updating
        let mut active_model: expense::ActiveModel = expense_exists.unwrap().into();

        // Update fields if they are provided in the DTO
        if let Some(description) = dto.description {
            active_model.description = Set(description);
        }
        if let Some(expense_type_id) = dto.expense_type_id {
            active_model.expense_type_id = Set(expense_type_id.as_bytes().to_vec());
            
        }
        

        // Save the updated expense to the database
        let updated_expense = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("FOREIGN KEY") {
                        return RepositoryError::OperationFailed(
                            "Invalid expense type ID".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(updated_expense)
    }

    async fn delete(&self,user_id: Uuid, expense_id : Uuid) 
    -> Result<(), RepositoryError>
    {
        // Check if the expense is in use in the transaction table
        let is_in_use = self
            .is_in_use_in_transaction(user_id, expense_id)
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if is_in_use {
            return Err(RepositoryError::OperationFailed(format!(
                "Expense with ID {} is currently in use in a transaction and cannot be deleted",
                expense_id
            )));
        }

        // Ensure the expense belongs to the user
        let expense_exists = expense::Entity::find()
            .filter(expense::Column::Id.eq(expense_id.as_bytes().to_vec())) // Filter by expense ID
            .filter(expense::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if expense_exists.is_none() {
            return Err(RepositoryError::NotFound(format!(
                "Expense with ID {} not found for user {}",
                expense_id, user_id
            )));
        }

        // Delete the expense
        expense::Entity::delete_many()
            .filter(expense::Column::Id.eq(expense_id.as_bytes().to_vec())) // Filter by expense ID
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(())
    }
}


#[async_trait::async_trait]
impl ExpenseRepositoryUtill for ExpenseRepositoryImpl {
    async fn find_all_by_user_id(&self, user_id: Uuid) 
    -> Result<Vec<expense::Model>, RepositoryError>
    {
        // Query the database to retrieve all expenses for the given user
        let expenses = expense::Entity::find()
            .filter(expense::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of expenses
        Ok(expenses)
    }


    async fn find_by_user_id_and_expense_id(&self, user_id: Uuid, expense_id: Uuid) 
    -> Result<Option<expense::Model>, RepositoryError>
    {
        // Query the database to find the expense by ID and ensure it belongs to the user
        let expense = expense::Entity::find()
            .filter(expense::Column::Id.eq(expense_id.as_bytes().to_vec())) // Filter by expense ID
            .filter(expense::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the expense if found, or None if not found
        Ok(expense)
    }


    async fn find_by_user_id_and_expense_type_id(&self, user_id: uuid::Uuid, expense_id: Uuid) 
    -> Result<Option<expense::Model>, RepositoryError>
    {
        // Query the database to find the expense by expense_type_id and ensure it belongs to the user
        let expense = expense::Entity::find()
            .filter(expense::Column::ExpenseTypeId.eq(expense_id.as_bytes().to_vec())) // Filter by expense type ID
            .filter(expense::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the expense if found, or None if not found
        Ok(expense)
    }
    async fn is_in_use_in_transaction(&self, user_id: Uuid, expense_id: Uuid) 
    -> Result<bool, RepositoryError>
    {
        // Query the database to check if the expense is in use in the transaction table
        let is_in_use = transaction::Entity::find()
            .filter(transaction::Column::ExpenseId.eq(expense_id.as_bytes().to_vec())) // Filter by expense ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .count(self.db_pool.as_ref()) // Count matching records
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return true if the expense is in use, otherwise false
        Ok(is_in_use > 0)
    }
}