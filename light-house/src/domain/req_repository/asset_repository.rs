use uuid::Uuid;

use crate::{domain::{dto::asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto}, entities::asset}, soc::soc_repository::RepositoryError};


#[async_trait::async_trait]
#[mockall::automock]
pub trait AssetRepositoryUtility: Send + Sync {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset::Model>, String>;
    
}


#[async_trait::async_trait]
#[mockall::automock]
pub trait AssetRepositoryBase: Send + Sync {
    async fn create(&self, user_id: Uuid, dto: ReqCreateAssetDto) -> Result<asset::Model, RepositoryError>;
    async fn find_by_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<Option<asset::Model>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<asset::Model>, RepositoryError>;
    async fn update(&self, dto: ReqUpdateAssetDto, user_id: Uuid, asset_id: Uuid) -> Result<asset::Model, RepositoryError>;
    async fn delete(&self,user_id: Uuid, asset_id : Uuid) -> Result<(), RepositoryError>;
}