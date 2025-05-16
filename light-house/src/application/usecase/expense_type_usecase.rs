use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::expense_type_usecase::ExpenseTypeUsecase, domain::{dto::expense_type_dto::{ReqCreateExpenseTypeDto, ReqUpdateExpenseTypeDto, ResEntryExpenseTypeDto, ResListExpenseTypeDto}, req_repository::expense_type_repository::{ExpenseTypeRepositoryBase, ExpenseTypeRepositoryUtility}}, soc::soc_usecase::UsecaseError};






pub struct ExpenseTypeUseCase<T>
where
    T: ExpenseTypeRepositoryBase + ExpenseTypeRepositoryUtility + Send + Sync,
{
    expense_type_repo: Arc<T>,
}
impl<T> ExpenseTypeUseCase<T>
where
    T: ExpenseTypeRepositoryBase + ExpenseTypeRepositoryUtility + Send + Sync,
{
    pub fn new(expense_type_repo: Arc<T>) -> Self {
        Self { expense_type_repo }
    }
}



#[async_trait::async_trait]
impl<T> ExpenseTypeUsecase for ExpenseTypeUseCase<T>
where
    T: ExpenseTypeRepositoryBase + ExpenseTypeRepositoryUtility + Send + Sync,
{
    async fn create_expense_type(
        &self, 
        user_id: Uuid, 
        expense_type_dto: ReqCreateExpenseTypeDto
    ) 
        -> Result<ResEntryExpenseTypeDto, UsecaseError>
    {
        match self.expense_type_repo.create(user_id, expense_type_dto).await {
            Ok(expense_type) => {
                let res_entry = ResEntryExpenseTypeDto {
                    id: match String::from_utf8(expense_type.id) {
                        Ok(id) => id, // Convert the ID from Vec<u8> to String
                        Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UTF-8 error
                    },
                    name: expense_type.name, // Expense type name
                    created_at: match expense_type.created_at {
                        Some(dt) => dt.to_string(), // Convert created_at to String if present
                        None => String::from(""),   // Default to an empty string if None
                    },
                    updated_at: match expense_type.updated_at {
                        Some(dt) => dt.to_string(), // Convert updated_at to String if present
                        None => String::from(""),   // Default to an empty string if None
                    },
                };
                return Ok(res_entry);
            },
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };
        
    }

    async fn get_expense_type(
        &self, 
        user_id: Uuid , 
        expense_type_id: Uuid
    ) 
        -> Result<Option<ResEntryExpenseTypeDto>, UsecaseError>
    {
        match self.expense_type_repo.find_by_user_id_and_expense_type_id(expense_type_id, user_id).await {
            Ok(expense_type) => {
                if let Some(expense_type) = expense_type {
                    let res_entry = ResEntryExpenseTypeDto {
                        id: match String::from_utf8(expense_type.id) {
                            Ok(id) => id, // Convert the ID from Vec<u8> to String
                            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UTF-8 error
                        },
                        name: expense_type.name, // Expense type name
                        created_at: match expense_type.created_at {
                            Some(dt) => dt.to_string(), // Convert created_at to String if present
                            None => String::from(""),   // Default to an empty string if None
                        },
                        updated_at: match expense_type.updated_at {
                            Some(dt) => dt.to_string(), // Convert updated_at to String if present
                            None => String::from(""),   // Default to an empty string if None
                        },
                    };
                    return Ok(Some(res_entry));
                }
                return Ok(None);
            },
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };
    }

    async fn update_expense_type(
        &self, 
        user_id: Uuid,  
        expense_type_id: Uuid, 
        expense_type_dto: ReqUpdateExpenseTypeDto
    ) 
        -> Result<ResEntryExpenseTypeDto, UsecaseError>
    {
        match self.expense_type_repo.update(expense_type_dto, user_id, expense_type_id).await {
            Ok(expense_type) => {
                let res_entry = ResEntryExpenseTypeDto {
                    id: match String::from_utf8(expense_type.id) {
                        Ok(id) => id, // Convert the ID from Vec<u8> to String
                        Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UTF-8 error
                    },
                    name: expense_type.name, // Expense type name
                    created_at: match expense_type.created_at {
                        Some(dt) => dt.to_string(), // Convert created_at to String if present
                        None => String::from(""),   // Default to an empty string if None
                    },
                    updated_at: match expense_type.updated_at {
                        Some(dt) => dt.to_string(), // Convert updated_at to String if present
                        None => String::from(""),   // Default to an empty string if None
                    },
                };
                return Ok(res_entry);
            },
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };
    }

    async fn delete_expense_type(
        &self, 
        user_id: Uuid , 
        expense_type_id: Uuid
    ) 
        -> Result<(), UsecaseError>
    {
        match self.expense_type_repo.delete(user_id, expense_type_id).await {
            Ok(_) => return Ok(()), // Successfully deleted
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        }
    }

    async fn get_all_expense_type(
        &self, 
        user_id: Uuid
    ) 
        -> Result<ResListExpenseTypeDto, UsecaseError>
    {
        match self.expense_type_repo.find_all_by_user_id(user_id).await {
            Ok(expense_types) => {
                let mut data = Vec::new();
                for expense_type in expense_types {
                    let id = match String::from_utf8(expense_type.id) {
                        Ok(id) => id, // Convert the ID from Vec<u8> to String
                        Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UTF-8 error
                    };
                    let created_at = match expense_type.created_at {
                        Some(dt) => dt.to_string(), // Convert created_at to String if present
                        None => String::from(""),   // Default to an empty string if None
                    };
                    let updated_at = match expense_type.updated_at {
                        Some(dt) => dt.to_string(), // Convert updated_at to String if present
                        None => String::from(""),   // Default to an empty string if None
                    };
                    let res_entry = ResEntryExpenseTypeDto {
                        id,
                        name: expense_type.name,
                        created_at,
                        updated_at,
                    };
                    data.push(res_entry);
                }
                return Ok(ResListExpenseTypeDto { length: data.len() as i32, data });
            },
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        }
    }
}