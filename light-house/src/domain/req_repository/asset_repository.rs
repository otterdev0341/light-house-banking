use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::asset;


#[async_trait::async_trait]
pub trait AssetRepository: Send + Sync {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset::Entity>, String>;
    
}