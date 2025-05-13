use uuid::Uuid;

use crate::{domain::{dto::contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto}, entities::contact_type}, soc::soc_repository::RepositoryError};



#[async_trait::async_trait]
pub trait ContactTypeRepositoryUtility: Send + Sync {
    async fn find_by_name(&self, name: &str, user_id: Uuid) -> Result<Option<contact_type::Model>, String>;
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<contact_type::Model>, String>;
}


#[async_trait::async_trait]
#[mockall::automock]
pub trait ContactTypeRepositoryBase: Send + Sync {
    async fn create(&self, user_id: Uuid, dto: ReqCreateContactTypeDto) -> Result<contact_type::Model, RepositoryError>;
    async fn find_by_id(&self, user_id: Uuid, contact_type_id: Uuid) -> Result<Option<contact_type::Model>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<contact_type::Model>, RepositoryError>;
    async fn update(&self, dto: ReqUpdateContactTypeDto, user_id: Uuid, contact_type_id: Uuid) -> Result<contact_type::Model, RepositoryError>;
    async fn delete(&self,user_id: Uuid, contact_type_id : Uuid) -> Result<(), RepositoryError>;
}