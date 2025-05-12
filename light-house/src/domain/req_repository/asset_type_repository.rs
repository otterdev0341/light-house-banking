use uuid::Uuid;

use crate::domain::entities::asset_type;





#[async_trait::async_trait]
pub trait AssetTypeRepository: Send + Sync {
    async fn is_in_user(&self, user_id: Uuid, asset_type_id: Uuid) -> Result<bool, String>;
    async fn fine_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset_type::Entity>, String>;
}