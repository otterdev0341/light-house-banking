use uuid::Uuid;

use crate::{domain::{dto::contact_dto::{ReqCreateContactDto, ReqUpdateContactDto}, entities::{contact, contact_type}}, soc::soc_repository::RepositoryError};




#[async_trait::async_trait]
#[mockall::automock]
pub trait ContactRepositoryUtility: Send + Sync {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<contact::Model>, RepositoryError>;
    async fn find_by_user_id_and_contact_id(&self, user_id: Uuid, contact_id: Uuid) -> Result<Option<contact::Model>, RepositoryError>;
    async fn find_by_user_id_and_contact_type_id(&self, user_id: uuid::Uuid, contact_id: Uuid) -> Result<Option<contact::Model>, RepositoryError>;
    async fn is_in_use_in_transaction(&self, user_id: Uuid, contact_id: Uuid) -> Result<bool, RepositoryError>;
    async fn find_contact_type_by_id(&self, user_id: Uuid, contact_type_id: Uuid) -> Result<Option<contact_type::Model>, RepositoryError>;
}



#[async_trait::async_trait]
#[mockall::automock]
pub trait ContactRepositoryBase: Send + Sync {
    async fn create(&self, user_id: Uuid, dto: ReqCreateContactDto) -> Result<contact::Model, RepositoryError>;
    async fn find_by_id(&self, contact_id: Uuid) -> Result<Option<contact::Model>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<contact::Model>, RepositoryError>;
    async fn update(&self, dto: ReqUpdateContactDto, user_id: Uuid, contact_id: Uuid) -> Result<contact::Model, RepositoryError>;
    async fn delete(&self,user_id: Uuid, contact_id : Uuid) -> Result<(), RepositoryError>;
}