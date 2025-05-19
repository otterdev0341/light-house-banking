use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{dto::expense_dto::{ReqCreateExpenseDto, ReqUpdateExpenseDto}, entities::{expense, expense_type, transaction}, req_repository::expense_repository::{ExpenseRepositoryBase, ExpenseRepositoryUtill}}, soc::soc_repository::RepositoryError};





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
    -> Result<expense::Model, RepositoryError> {
        // Validate and convert expense_type_id
        let expense_type_id = Uuid::parse_str(&dto.expense_type_id)
            .map_err(|err| RepositoryError::InvalidInput(format!("Invalid expense_type_id: {}", err)))?
            .as_bytes()
            .to_vec();

        // Create the ActiveModel for the expense
        let new_expense = expense::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the expense
            description: Set(dto.description),          // Set the description from the DTO
            expense_type_id: Set(expense_type_id),      // Set the validated expense type ID
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

    async fn update(
        &self, 
        user_id: Uuid, 
        expense_id: Uuid,
        dto: ReqUpdateExpenseDto, 
    ) -> Result<expense::Model, RepositoryError> {
        // Step 1: Validate and convert `expense_type_id` if provided
        let expense_type_id = if let Some(expense_type_id_str) = &dto.expense_type_id {
            // Parse the `expense_type_id` string into a UUID
            let expense_type_id = Uuid::parse_str(expense_type_id_str)
                .map_err(|err| RepositoryError::InvalidInput(format!("Invalid expense_type_id: {}", err)))?
                .as_bytes()
                .to_vec();

            // Step 2: Check if the `expense_type_id` exists in the `expense_type` table
            let expense_type_exists = expense_type::Entity::find()
                .filter(expense_type::Column::Id.eq(expense_type_id.clone())) // Filter by `expense_type_id`
                .filter(expense_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
                .one(self.db_pool.as_ref())
                .await
                .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

            // If the `expense_type_id` does not exist, return an error
            if expense_type_exists.is_none() {
                return Err(RepositoryError::InvalidInput(format!(
                    "Expense type with ID '{}' does not exist",
                    expense_type_id_str
                )));
            }

            // Return the validated and converted `expense_type_id`
            Some(expense_type_id)
        } else {
            // If `expense_type_id` is not provided, set it to `None`
            None
        };

        // Step 3: Create an `ActiveModel` for the expense
        let mut active_model = expense::ActiveModel {
            id: Set(expense_id.as_bytes().to_vec()), // Set the expense ID
            user_id: Set(user_id.as_bytes().to_vec()), // Set the user ID
            ..Default::default() // Initialize other fields with default values
        };

        // Step 4: Update the `description` field if provided in the DTO
        if let Some(description) = dto.description {
            active_model.description = Set(description);
        }

        // Step 5: Update the `expense_type_id` field if it was validated earlier
        if let Some(expense_type_id) = expense_type_id {
            active_model.expense_type_id = Set(expense_type_id);
        }

        // Step 6: Update the expense record in the database
        let updated_expense = active_model
            .update(self.db_pool.as_ref()) // Perform the update operation
            .await
            .map_err(|err| {
                // Handle foreign key constraint errors
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("FOREIGN KEY") {
                        return RepositoryError::InvalidInput(
                            "Invalid expense_type_id: does not satisfy foreign key constraint".to_string(),
                        );
                    }
                }
                // Handle other database errors
                RepositoryError::DatabaseError(err.to_string())
            })?;

        // Step 7: Return the updated expense record
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

    async fn find_expense_type_by_id(&self, expense_type_id: Uuid) -> Result<Option<expense_type::Model>, RepositoryError>
    {
        // Query the database to find the expense type by ID
        let expense_type = expense_type::Entity::find()
            .filter(expense_type::Column::Id.eq(expense_type_id.as_bytes().to_vec())) // Filter by expense type ID
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the expense type if found, or None if not found
        Ok(expense_type)
    }
}