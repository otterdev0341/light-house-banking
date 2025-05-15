use uuid::Uuid;

use crate::{domain::dto::expense_dto::{ReqCreateExpenseDto, ReqUpdateExpenseDto, ResEntryExpenseDto, ResListExpenseDto}, soc::soc_usecase::UsecaseError};




#[async_trait::async_trait]
pub trait ExpenseUsecase {
    async fn create_expense(&self, user_id: Uuid, expense_dto: ReqCreateExpenseDto) -> Result<ResEntryExpenseDto, UsecaseError>;
    async fn get_expense(&self, user_id: Uuid , expense_id: Uuid) -> Result<Option<ResEntryExpenseDto>, UsecaseError>;
    async fn update_expense(&self, user_id: Uuid,  expense_id: Uuid, expense_dto: ReqUpdateExpenseDto) -> Result<ResEntryExpenseDto, UsecaseError>;
    async fn delete_expense(&self, user_id: Uuid , expense_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_expense(&self, user_id: Uuid) -> Result<ResListExpenseDto, UsecaseError>;
}