use uuid::Uuid;

use crate::domain::entities::contact;




#[async_trait::async_trait]
pub trait ContactRepository: Send + Sync {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<contact::Entity>, String>;
    async fn find_by_user_id_and_contact_type_name(&self, user_id: uuid::Uuid, contact_type_name: &str) -> Result<Option<crate::domain::entities::contact::Entity>, String>;
    async fn is_in_use_in_transaction(&self, user_id: Uuid, contact_id: Uuid) -> Result<bool, String>;
}