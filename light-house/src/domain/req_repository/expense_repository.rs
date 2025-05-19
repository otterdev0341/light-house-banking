use uuid::Uuid;

use crate::{domain::{dto::expense_dto::{ReqCreateExpenseDto, ReqUpdateExpenseDto}, entities::{expense, expense_type}}, soc::soc_repository::RepositoryError};


#[async_trait::async_trait]
#[mockall::automock]
pub trait ExpenseRepositoryUtill: Send + Sync {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<expense::Model>, RepositoryError>;
    async fn find_by_user_id_and_expense_id(&self, user_id: Uuid, expense_id: Uuid) -> Result<Option<expense::Model>, RepositoryError>;
    async fn find_by_user_id_and_expense_type_id(&self, user_id: uuid::Uuid, expense_id: Uuid) -> Result<Option<expense::Model>, RepositoryError>;
    async fn is_in_use_in_transaction(&self, user_id: Uuid, expense_id: Uuid) -> Result<bool, RepositoryError>;
    async fn find_expense_type_by_id(&self, expense_type_id: Uuid) -> Result<Option<expense_type::Model>, RepositoryError>;
}

#[async_trait::async_trait]
#[mockall::automock]
pub trait ExpenseRepositoryBase: Send + Sync {
    async fn create(&self, user_id: Uuid, dto: ReqCreateExpenseDto) -> Result<expense::Model, RepositoryError>;
    async fn find_by_id(&self, expense_id: Uuid) -> Result<Option<expense::Model>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<expense::Model>, RepositoryError>;
    async fn update(&self, user_id: Uuid,  expense_id: Uuid, dto: ReqUpdateExpenseDto) -> Result<expense::Model, RepositoryError>;
    async fn delete(&self,user_id: Uuid, expense_id : Uuid) -> Result<(), RepositoryError>;
}