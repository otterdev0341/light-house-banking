use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::expense_usecase::ExpenseUsecase, domain::{dto::expense_dto::{ReqCreateExpenseDto, ReqUpdateExpenseDto, ResEntryExpenseDto, ResListExpenseDto}, req_repository::expense_repository::{ExpenseRepositoryBase, ExpenseRepositoryUtill}}, soc::soc_usecase::UsecaseError};






pub struct ExpenseUseCase<T>
where
    T: ExpenseRepositoryBase + ExpenseRepositoryUtill + Send + Sync,
{
    expense_repo: Arc<T>,
}

impl<T> ExpenseUseCase<T>
where
    T: ExpenseRepositoryBase + ExpenseRepositoryUtill + Send + Sync,
{
    pub fn new(expense_repo: Arc<T>) -> Self {
        Self { expense_repo }
    }
}


#[async_trait::async_trait]
impl<T> ExpenseUsecase for ExpenseUseCase<T>
where 
    T: ExpenseRepositoryBase + ExpenseRepositoryUtill + Send + Sync,
{
    async fn create_expense(
        &self, 
        user_id: Uuid, 
        expense_dto: ReqCreateExpenseDto
    ) -> Result<ResEntryExpenseDto, UsecaseError> {
        // Step 1: Create the expense in the database
        let expense_created = match self.expense_repo.create(user_id, expense_dto).await {
            Ok(expense) => expense,
            Err(err) => {
                log::error!("Failed to create expense: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        // Step 2: Fetch the expense type name using the expense_type_id
        let expense_type_name = match self
            .expense_repo
            .find_by_id(Uuid::from_slice(&expense_created.expense_type_id).map_err(|err| UsecaseError::InvalidData(err.to_string()))?)
            .await
        {
            Ok(Some(expense_type)) => expense_type.description,
            Ok(None) => String::from("Unknown"), // Default value if expense type is not found
            Err(err) => {
                log::error!("Failed to fetch expense type: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        // Step 3: Map the result to ResEntryExpenseDto
        let res_entry = ResEntryExpenseDto {
            id: match Uuid::from_slice(&expense_created.id) {
                Ok(id) => id.to_string(),
                Err(err) => return Err(UsecaseError::InvalidData(err.to_string())),
            },
            description: expense_created.description,
            expense_type_name,
            created_at: match expense_created.created_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
            updated_at: match expense_created.updated_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
        };

        // Step 4: Return the response object
        Ok(res_entry)
    }

    async fn get_expense(
        &self, 
        user_id: Uuid , 
        expense_id: Uuid
    ) 
        -> Result<Option<ResEntryExpenseDto>, UsecaseError>
    {
        // Step 1: Fetch the expense by user_id and expense_id from the repository
        let expense = match self.expense_repo.find_by_user_id_and_expense_id(user_id, expense_id).await {
            Ok(Some(expense)) => {
                // Step 2: Fetch the expense type name using the expense_type_id
                let expense_type_name = match self
                    .expense_repo
                    .find_by_id(Uuid::from_slice(&expense.expense_type_id).map_err(|err| UsecaseError::Unexpected(err.to_string()))?)
                    .await
                {
                    Ok(Some(expense_type)) => expense_type.description,
                    Ok(None) => String::from("Unknown"),
                    Err(err) => return Err(UsecaseError::from(err)),
                };

                // Step 3: Map the expense details to ResEntryExpenseDto
                Some(ResEntryExpenseDto {
                    id: match Uuid::from_slice(&expense.id) {
                        Ok(id) => id.to_string(),
                        Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
                    },
                    description: expense.description,
                    expense_type_name,
                    created_at: match expense.created_at {
                        Some(dt) => dt.to_string(),
                        None => String::from(""),
                    },
                    updated_at: match expense.updated_at {
                        Some(dt) => dt.to_string(),
                        None => String::from(""),
                    },
                })
            }
            Ok(None) => return Ok(None),
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 4: Return the mapped expense details
        Ok(expense)
    }

    async fn update_expense(
        &self, 
        user_id: Uuid,  
        expense_id: Uuid, 
        expense_dto: ReqUpdateExpenseDto
    ) -> Result<ResEntryExpenseDto, UsecaseError> {
        // Step 1: Call the repository to update the expense
        let updated_expense = self
            .expense_repo
            .update(user_id, expense_id, expense_dto)
            .await
            .map_err(|err| {
                log::error!("Failed to update expense: {}", err);
                UsecaseError::from(err)
            })?;

        // Step 2: Fetch the expense type name using the expense_type_id
        let expense_type_id = Uuid::from_slice(&updated_expense.expense_type_id)
            .map_err(|err| {
                log::error!("Invalid expense_type_id: {}", err);
                UsecaseError::InvalidData(format!("Invalid expense_type_id: {}", err))
            })?;

        let expense_type_name = match self.expense_repo.find_expense_type_by_id(expense_type_id).await {
            Ok(Some(expense_type)) => expense_type.name,
            Ok(None) => {
                log::warn!(
                    "No expense type found for expense_type_id: {}",
                    expense_type_id
                );
                String::from("Unknown")
            },
            Err(err) => {
                log::error!("Failed to fetch expense type: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        // Step 3: Map the result to ResEntryExpenseDto
        let res_entry = ResEntryExpenseDto {
            id: Uuid::from_slice(&updated_expense.id)
                .map_err(|err| {
                    log::error!("Invalid expense ID: {}", err);
                    UsecaseError::InvalidData(err.to_string())
                })?
                .to_string(),
            description: updated_expense.description,
            expense_type_name,
            created_at: match updated_expense.created_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
            updated_at: match updated_expense.updated_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
        };

        log::debug!("Expense updated successfully: {:?}", res_entry);

        // Step 4: Return the response object
        Ok(res_entry)
    }

    async fn delete_expense(
        &self, 
        user_id: Uuid , 
        expense_id: Uuid
    ) 
        -> Result<(), UsecaseError>
    {
        // Step 1: Check if the expense exists
        let expense_exists = match self.expense_repo.find_by_user_id_and_expense_id(user_id, expense_id).await {
            Ok(Some(_)) => true,
            Ok(None) => {
                return Err(UsecaseError::ResourceNotFound(format!(
                    "Expense with ID '{}' not found",
                    expense_id
                )))
            }
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Delete the expense if it exists
        if expense_exists {
            match self.expense_repo.delete(user_id, expense_id).await {
                Ok(_) => Ok(()),
                Err(err) => Err(UsecaseError::from(err)),
            }
        } else {
            Err(UsecaseError::ResourceNotFound(format!(
                "Expense with ID '{}' not found",
                expense_id
            )))
        }
    }

    async fn get_all_expense(
        &self, 
        user_id: Uuid
    ) 
        -> Result<ResListExpenseDto, UsecaseError>
    {
        // Step 1: Fetch all expenses for the user from the repository
        let expenses = match self.expense_repo.find_all_by_user_id(user_id).await {
            Ok(expenses) => expenses,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Map the expenses to ResEntryExpenseDto
        let mut data = Vec::new();
        for expense in expenses {
            // Fetch the expense type name using the expense_type_id
            let expense_type_name = match self
                .expense_repo
                .find_by_user_id_and_expense_id(user_id, Uuid::from_slice(&expense.expense_type_id).map_err(|err| UsecaseError::Unexpected(err.to_string()))?)
                .await
            {
                Ok(Some(expense_type)) => expense_type.description,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            let res_entry = ResEntryExpenseDto {
                id: match Uuid::from_slice(&expense.id) {
                    Ok(id) => id.to_string(),
                    Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
                },
                description: expense.description,
                expense_type_name,
                created_at: match expense.created_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                },
                updated_at: match expense.updated_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                },
            };

            data.push(res_entry);
        }

        // Step 3: Create the response object
        let res_list = ResListExpenseDto {
            length: data.len() as i32,
            data,
        };

        // Step 4: Return the response object
        Ok(res_list)
    }
}

