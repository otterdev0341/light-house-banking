use uuid::Uuid;

use crate::{domain::dto::contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto}, soc::soc_usecase::UsecaseError};







#[async_trait::async_trait]
pub trait ContactTypeUsecase {
    async fn create_contact_type(&self, user_id: Uuid, contact_type_dto: ReqCreateContactTypeDto) -> Result<ResEntryContactTypeDto, UsecaseError>;
    async fn get_contact_type(&self, user_id: Uuid , contact_type_id: Uuid) -> Result<Option<ResEntryContactTypeDto>, UsecaseError>;
    async fn update_contact_type(&self, user_id: Uuid,  contact_type_id: Uuid, contact_type_dto: ReqUpdateContactTypeDto) -> Result<ResEntryContactTypeDto, UsecaseError>;
    async fn delete_contact_type(&self, user_id: Uuid , contact_type_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_contact_type(&self, user_id: Uuid) -> Result<ResListContactTypeDto, UsecaseError>;
}