use uuid::Uuid;

use crate::{domain::{dto::transaction_dto::{ReqCreateIncomeDto, ReqCreatePaymentDto, ReqCreateTransferDto, ReqUpdateIncomeDto, ReqUpdatePaymentDto, ReqUpdateTransferDto, ResEntryTransferDto}, entities::transaction}, soc::soc_repository::RepositoryError};



// handler all tranfer feature
#[async_trait::async_trait]
#[mockall::automock]
pub trait TranferRepositoryUtility{
    fn create_transfer(&self, user_id: Uuid, tranfer_dto: ReqCreateTransferDto) -> Result<transaction::Model, RepositoryError>;
    fn update_transfer(&self, user_id: Uuid, transaction_id: Uuid, tranfer_dto: ReqUpdateTransferDto) -> Result<transaction::Model, RepositoryError>;
    fn delete_transfer(&self, user_id: Uuid, transaction_id: Uuid) -> Result<(), RepositoryError>;
    fn get_transfer_by_id(&self, user_id: Uuid, transaction_id: Uuid) -> Result<Option<transaction::Model>, RepositoryError>;
    fn get_all_transfers_by_user(&self, user_id: Uuid) -> Result<Vec<transaction::Model>, RepositoryError>;
}





#[async_trait::async_trait]
#[mockall::automock]
pub trait RecordIncomeRepositoryUtility {
    fn create_income_record(&self, user_id: Uuid, income_record_dto: ReqCreateIncomeDto) -> Result<transaction::Model, RepositoryError>;
    fn update_income_record(&self, user_id: Uuid, transaction_id: Uuid, income_record_dto: ReqUpdateIncomeDto) -> Result<transaction::Model, RepositoryError>;
    fn delete_income_record(&self, user_id: Uuid, transaction_id: Uuid) -> Result<(), RepositoryError>;
    fn get_income_record_by_id(&self, user_id: Uuid, transaction_id: Uuid) -> Result<Option<transaction::Model>, RepositoryError>;
    fn get_all_income_record_by_user(&self, user_id: Uuid) -> Result<Vec<transaction::Model>, RepositoryError>;
}



#[async_trait::async_trait]
#[mockall::automock]
pub trait RecordPaymentRepositoryUtility {
    fn create_payment_record(&self, user_id: Uuid, payment_record_dto: ReqCreatePaymentDto) -> Result<transaction::Model, RepositoryError>;
    fn update_payment_record(&self, user_id: Uuid, transaction_id: Uuid, payment_record_dto: ReqUpdatePaymentDto) -> Result<transaction::Model, RepositoryError>;
    fn delete_payment_record(&self, user_id: Uuid, transaction_id: Uuid) -> Result<(), RepositoryError>;
    fn get_payment_record_by_id(&self, user_id: Uuid, transaction_id: Uuid) -> Result<Option<transaction::Model>, RepositoryError>;
    fn get_all_payment_record_by_user(&self, user_id: Uuid) -> Result<Vec<transaction::Model>, RepositoryError>;
}


