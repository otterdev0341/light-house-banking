use uuid::Uuid;

use crate::{domain::dto::transaction_type_dto::ResListTransactionTypeDto, soc::soc_usecase::UsecaseError};



#[async_trait::async_trait]
pub trait TransactionTypeUsecase {
    async fn get_all_transaction_type(&self, user_id: Uuid) -> Result<ResListTransactionTypeDto, UsecaseError>;
}