use uuid::Uuid;

use crate::{domain::entities::gender, soc::soc_repository::RepositoryError};



#[async_trait::async_trait]
pub trait GenderRepository: Send + Sync {
    
    async fn get_gender_by_id(&self, gender_id: Uuid) -> Result<Option<gender::Model>, RepositoryError>;
    async fn get_all_gender(&self) -> Result<Vec<gender::Model>, RepositoryError>;
}