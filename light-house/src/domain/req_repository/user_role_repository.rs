use uuid::Uuid;

use crate::{domain::entities::user_role, soc::soc_repository::RepositoryError};



#[async_trait::async_trait]
#[mockall::automock]
pub trait RoleManagementRepository {
    async fn has_role(&self, user_id: Uuid, role: &str) -> Result<bool, RepositoryError>;
    async fn assign_role(&self, admin_id: Uuid, target_user_id: Uuid, role: &str) -> Result<(), RepositoryError>;
    async fn revoke_role(&self, admin_id: Uuid, target_user_id: Uuid, role: &str) -> Result<(), RepositoryError>;
    async fn get_role_by_id(&self, role_id: Uuid) -> Result<Option<user_role::Model>, RepositoryError>;
}