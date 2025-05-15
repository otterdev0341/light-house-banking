use uuid::Uuid;

use crate::{domain::dto::contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto}, soc::soc_usecase::UsecaseError};




#[async_trait::async_trait]
pub trait ContactUsecase {
    async fn create_contact(&self, user_id: Uuid, contact_dto: ReqCreateContactDto) -> Result<ResEntryContactDto, UsecaseError>;
    async fn get_contact(&self, user_id: Uuid , contact_id: Uuid) -> Result<Option<ResEntryContactDto>, UsecaseError>;
    async fn update_contact(&self, user_id: Uuid,  contact_id: Uuid, contact_dto: ReqUpdateContactDto) -> Result<ResEntryContactDto, UsecaseError>;
    async fn delete_contact(&self, user_id: Uuid , contact_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_contact(&self, user_id: Uuid) -> Result<ResListContactDto, UsecaseError>;
}