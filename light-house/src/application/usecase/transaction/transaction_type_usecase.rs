use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::transaction_type_usecase::TransactionTypeUsecase, domain::{dto::transaction_type_dto::{ResEntryTransactionTypeDto, ResListTransactionTypeDto}, req_repository::transaction_repository::TransactionTypeRepositoryUtility}, soc::soc_usecase::UsecaseError};



pub struct TransactionTypeUseCase<T>
where
    T: TransactionTypeRepositoryUtility + Send + Sync,
{
    transaction_type_repository: Arc<T>,
}


impl<T> TransactionTypeUseCase<T>
where
    T: TransactionTypeRepositoryUtility + Send + Sync,
{
    pub fn new(transaction_type_repository: Arc<T>) -> Self {
        Self { transaction_type_repository }
    }
}

#[async_trait::async_trait]
impl<T> TransactionTypeUsecase for TransactionTypeUseCase<T>
where
    T: TransactionTypeRepositoryUtility + Send + Sync,
{
    async fn get_all_transaction_type(&self, _user_id: Uuid) -> Result<ResListTransactionTypeDto, UsecaseError>
    {
        // Step 1: Fetch all transaction types from the repository
        let transaction_types = match self.transaction_type_repository.get_all_transaction_types_by_user().await {
            Ok(types) => types,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Map the result to ResListTransactionTypeDto
        let mut collect_transaction_types: Vec<ResEntryTransactionTypeDto> = Vec::new();
        for transaction_type in transaction_types {
            let res_enty = ResEntryTransactionTypeDto {
                id: match Uuid::from_slice(&transaction_type.id) {
                    Ok(id) => id.to_string(),
                    Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
                },
                name: transaction_type.name,
                created_at: match transaction_type.created_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                },
                updated_at: match transaction_type.updated_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                },
            };
            collect_transaction_types.push(res_enty);
        }

        return Ok(ResListTransactionTypeDto {
            length: collect_transaction_types.len() as i32,
            data: collect_transaction_types,
        });

        
    }
}
 