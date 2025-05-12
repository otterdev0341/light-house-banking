use crate::domain::entities::contact_type;



#[async_trait::async_trait]
pub trait ContactTypeRepository: Send + Sync {
    async fn fine_by_name(&self, name: &str) -> Result<Option<contact_type::Entity>, String>;
    
}