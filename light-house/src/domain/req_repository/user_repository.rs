use uuid::Uuid;

use crate::{domain::{dto::auth_dto::{ReqSignUpDto, ReqUpdateUserDto}, entities::user}, soc::soc_repository::RepositoryError};




// done impl in to repostiroy_impl

#[async_trait::async_trait]
#[mockall::automock]
pub trait UserRepositoryUtility: Send + Sync {
    // check is data unique before create
    async fn find_by_username(&self, name: &str) -> Result<Option<user::Model>, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<user::Model>, RepositoryError>;
}

#[async_trait::async_trait]
#[mockall::automock]
pub trait UserRepositoryBase: Send + Sync {
    async fn create(&self, dto: ReqSignUpDto) -> Result<user::Model, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<user::Model>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<user::Model>, RepositoryError>;
    async fn update(&self, dto: ReqUpdateUserDto, user_id: Uuid) -> Result<user::Model, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}


#[async_trait::async_trait]
#[mockall::automock]
pub trait McpRepositoryBase: Send + Sync {
    async fn get_user_id_from_mcp_token(&self, mcp_token: &str) -> Result<user::Model, RepositoryError>;
    async fn regenerate_mcp_token(&self, user_id: Uuid) -> Result<(), RepositoryError>;
}