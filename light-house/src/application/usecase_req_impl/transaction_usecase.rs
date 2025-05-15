use uuid::Uuid;

use crate::{domain::dto::transaction_dto::{ReqCreateIncomeDto, ReqCreatePaymentDto, ReqCreateTransferDto, ReqUpdateIncomeDto, ReqUpdatePaymentDto, ReqUpdateTransferDto, ResEntryIncomeDto, ResEntryPaymentDto, ResEntryTransferDto, ResListIncomeDto, ResListPaymentDto, ResListTransferDto}, soc::soc_usecase::UsecaseError};




#[async_trait::async_trait]
pub trait TransferUsecase {
    async fn create_transfer(&self, user_id: Uuid, tranfer_dto: ReqCreateTransferDto) -> Result<ResEntryTransferDto, UsecaseError>;
    async fn get_transfer(&self, user_id: Uuid , transaction_id: Uuid) -> Result<Option<ResEntryTransferDto>, UsecaseError>;
    async fn update_transfer(&self, user_id: Uuid,  transaction_id: Uuid, tranfer_dto: ReqUpdateTransferDto) -> Result<ResEntryTransferDto, UsecaseError>;
    async fn delete_transfer(&self, user_id: Uuid , transaction_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_transfer(&self, user_id: Uuid) -> Result<ResListTransferDto, UsecaseError>;
}


#[async_trait::async_trait]
pub trait RecordIncomeUsecase {
    async fn create_income(&self, user_id: Uuid, income_dto: ReqCreateIncomeDto) -> Result<ResEntryIncomeDto, UsecaseError>;
    async fn get_income(&self, user_id: Uuid , transaction_id: Uuid) -> Result<Option<ResEntryIncomeDto>, UsecaseError>;
    async fn update_income(&self, user_id: Uuid,  transaction_id: Uuid, income_dto: ReqUpdateIncomeDto) -> Result<ResEntryIncomeDto, UsecaseError>;
    async fn delete_income(&self, user_id: Uuid , transaction_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_income(&self, user_id: Uuid) -> Result<ResListIncomeDto, UsecaseError>;
}


#[async_trait::async_trait]
pub trait RecordPaymentUsecase{
    async fn create_payment(&self, user_id: Uuid, payment_dto: ReqCreatePaymentDto) -> Result<ResEntryPaymentDto, UsecaseError>;
    async fn get_payment(&self, user_id: Uuid , transaction_id: Uuid) -> Result<Option<ResEntryPaymentDto>, UsecaseError>;
    async fn update_payment(&self, user_id: Uuid,  transaction_id: Uuid, payment_dto: ReqUpdatePaymentDto) -> Result<ResEntryPaymentDto, UsecaseError>;
    async fn delete_payment(&self, user_id: Uuid , transaction_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_payment(&self, user_id: Uuid) -> Result<ResListPaymentDto, UsecaseError>;
}