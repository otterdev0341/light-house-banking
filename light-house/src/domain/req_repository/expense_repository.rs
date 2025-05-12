use uuid::Uuid;

use crate::domain::entities::expense;


#[async_trait::async_trait]
pub trait ExpenseRepository: Send + Sync {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<expense::Entity>, String>;
}