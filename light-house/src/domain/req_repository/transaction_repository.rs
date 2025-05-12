use uuid::Uuid;

use crate::domain::entities::transaction;





#[async_trait::async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<transaction::Entity>, String>;
}