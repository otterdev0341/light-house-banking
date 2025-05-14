use uuid::Uuid;

use crate::{domain::{dto::transaction_dto::{ReqCreateIncomeDto, ReqCreatePaymentDto, ReqCreateTransferDto, ReqUpdateIncomeDto, ReqUpdatePaymentDto, ReqUpdateTransferDto, ResEntryTransferDto}, entities::transaction}, soc::soc_repository::RepositoryError};



// handler all tranfer feature
#[async_trait::async_trait]
#[mockall::automock]
pub trait TranferRepositoryUtility{
    async fn create_transfer(&self, user_id: Uuid, transfer_dto: ReqCreateTransferDto) -> Result<transaction::Model, RepositoryError>;
    async fn update_transfer(&self, user_id: Uuid, transaction_id: Uuid, transfer_dto: ReqUpdateTransferDto) -> Result<transaction::Model, RepositoryError>;
    async fn delete_transfer(&self, user_id: Uuid, transaction_id: Uuid) -> Result<(), RepositoryError>;
    async fn get_transfer_by_id(&self, user_id: Uuid, transaction_id: Uuid) -> Result<Option<transaction::Model>, RepositoryError>;
    async fn get_all_transfers_by_user(&self, user_id: Uuid) -> Result<Vec<transaction::Model>, RepositoryError>;
}





#[async_trait::async_trait]
#[mockall::automock]
pub trait RecordIncomeRepositoryUtility {
    async fn create_income_record(&self, user_id: Uuid, income_record_dto: ReqCreateIncomeDto) -> Result<transaction::Model, RepositoryError>;
    async fn update_income_record(&self, user_id: Uuid, transaction_id: Uuid, income_record_dto: ReqUpdateIncomeDto) -> Result<transaction::Model, RepositoryError>;
    async fn delete_income_record(&self, user_id: Uuid, transaction_id: Uuid) -> Result<(), RepositoryError>;
    async fn get_income_record_by_id(&self, user_id: Uuid, transaction_id: Uuid) -> Result<Option<transaction::Model>, RepositoryError>;
    async fn get_all_income_record_by_user(&self, user_id: Uuid) -> Result<Vec<transaction::Model>, RepositoryError>;
}



#[async_trait::async_trait]
#[mockall::automock]
pub trait RecordPaymentRepositoryUtility {
    async fn create_payment_record(&self, user_id: Uuid, payment_record_dto: ReqCreatePaymentDto) -> Result<transaction::Model, RepositoryError>;
    async fn update_payment_record(&self, user_id: Uuid, transaction_id: Uuid, payment_record_dto: ReqUpdatePaymentDto) -> Result<transaction::Model, RepositoryError>;
    async fn delete_payment_record(&self, user_id: Uuid, transaction_id: Uuid) -> Result<(), RepositoryError>;
    async fn get_payment_record_by_id(&self, user_id: Uuid, transaction_id: Uuid) -> Result<Option<transaction::Model>, RepositoryError>;
    async fn get_all_payment_record_by_user(&self, user_id: Uuid) -> Result<Vec<transaction::Model>, RepositoryError>;
}


