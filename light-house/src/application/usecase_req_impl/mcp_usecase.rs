use uuid::Uuid;

use crate::soc::soc_usecase::UsecaseError;




#[async_trait::async_trait]
#[mockall::automock]
pub trait McpRepositoryBaseUT: Send + Sync {
    async fn get_user_id_from_mcp_token(&self, mcp_token: &str) -> Result<Uuid, UsecaseError>;
    async fn regenerate_mcp_token(&self, user_id: Uuid) -> Result<(), UsecaseError>;
}