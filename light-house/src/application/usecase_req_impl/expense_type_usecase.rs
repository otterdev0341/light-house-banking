use uuid::Uuid;

use crate::{domain::dto::expense_type_dto::{ReqCreateExpenseTypeDto, ReqUpdateExpenseTypeDto, ResEntryExpenseTypeDto, ResListExpenseTypeDto}, soc::soc_usercase::UsecaseError};



#[async_trait::async_trait]
pub trait ExpenseTypeUsecase {
    async fn create_expense_type(&self, user_id: Uuid, expense_type_dto: ReqCreateExpenseTypeDto) -> Result<ResEntryExpenseTypeDto, UsecaseError>;
    async fn get_expense_type(&self, user_id: Uuid , expense_type_id: Uuid) -> Result<Option<ResEntryExpenseTypeDto>, UsecaseError>;
    async fn update_expense_type(&self, user_id: Uuid,  expense_type_id: Uuid, expense_type_dto: ReqUpdateExpenseTypeDto) -> Result<ResEntryExpenseTypeDto, UsecaseError>;
    async fn delete_expense_type(&self, user_id: Uuid , expense_type_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_expense_type(&self, user_id: Uuid) -> Result<ResListExpenseTypeDto, UsecaseError>;
}