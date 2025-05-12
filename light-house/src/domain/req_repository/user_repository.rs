use uuid::Uuid;

use crate::domain::{dto::auth_dto::ReqUpdateUserDto, entities::user};






#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_name(&self, name: &str) -> Result<Option<user::Entity>, String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<user::Entity>, String>;
    async fn update_profile(&self, dto: ReqUpdateUserDto, user_id: Uuid) -> Result<user::Entity, String>;
    async fn generate_mcp_token(&self, user_id: Uuid) -> Result<(), String>;
}