use uuid::Uuid;

use crate::{domain::{dto::assest_type_dto::{ReqCreateAssetTypeDto, ReqUpdateAssestTypeDto}, entities::asset_type}, soc::soc_repository::RepositoryError};





#[async_trait::async_trait]
pub trait AssetTypeRepositoryUtility: Send + Sync {
    async fn is_in_use(&self, user_id: Uuid, asset_type_id: Uuid) -> Result<bool, RepositoryError>;
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset_type::Model>, RepositoryError>;
    
}

#[async_trait::async_trait]
#[mockall::automock]
pub trait AssetTypeRepositoryBase: Send + Sync {
    async fn create(&self, user_id: Uuid, dto: ReqCreateAssetTypeDto) -> Result<asset_type::Model, RepositoryError>;
    async fn find_by_id(&self, user_id: Uuid, asset_type_id: Uuid) -> Result<Option<asset_type::Model>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<asset_type::Model>, RepositoryError>;
    async fn update(&self,user_id: Uuid, asset_type_id: Uuid, dto: ReqUpdateAssestTypeDto  ) -> Result<asset_type::Model, RepositoryError>;
    async fn delete(&self,user_id: Uuid, asset_type_id : Uuid) -> Result<(), RepositoryError>;
}