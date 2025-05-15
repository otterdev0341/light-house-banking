use uuid::Uuid;

use crate::{domain::{dto::expense_type_dto::{ReqCreateExpenseTypeDto, ReqUpdateExpenseTypeDto}, entities::expense_type}, soc::soc_repository::RepositoryError};





#[async_trait::async_trait]
pub trait ExpenseTypeRepositoryUtility: Send + Sync {
    async fn is_in_use(&self, user_id: Uuid, expesne_type_id: Uuid) -> Result<bool, RepositoryError>;
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<expense_type::Model>, RepositoryError>;
    async fn find_by_user_id_and_expense_type_id(&self, expesne_type_id: Uuid, user_id: Uuid) -> Result<Option<expense_type::Model>, RepositoryError>;
}

#[async_trait::async_trait]
#[mockall::automock]
pub trait ExpenseTypeRepositoryBase: Send + Sync {
    async fn create(&self, user_id: Uuid, dto: ReqCreateExpenseTypeDto) -> Result<expense_type::Model, RepositoryError>;
    async fn find_by_id(&self, expesne_type_id: Uuid) -> Result<Option<expense_type::Model>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<expense_type::Model>, RepositoryError>;
    async fn update(&self, dto: ReqUpdateExpenseTypeDto, user_id: Uuid, expesne_type_id: Uuid) -> Result<expense_type::Model, RepositoryError>;
    async fn delete(&self,user_id: Uuid, expesne_type_id : Uuid) -> Result<(), RepositoryError>;
}