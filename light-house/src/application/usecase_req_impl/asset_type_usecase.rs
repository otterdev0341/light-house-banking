use uuid::Uuid;

use crate::{domain::dto::assest_type_dto::{ReqCreateAssetTypeDto, ReqUpdateAssestTypeDto, ResEntryAssetTypeDto, ResListAssestTypeDto}, soc::soc_usecase::UsecaseError};




#[async_trait::async_trait]
pub trait AssetTypeUsecase {
    async fn create_asset_type(&self, user_id: Uuid, asset_type_dto: ReqCreateAssetTypeDto) -> Result<ResEntryAssetTypeDto, UsecaseError>;
    async fn get_asset_type(&self, user_id: Uuid ,asset_type_id: Uuid) -> Result<Option<ResEntryAssetTypeDto>, UsecaseError>;
    async fn update_asset_type(&self, user_id: Uuid, asset_type_id: Uuid, asset_type_dto: ReqUpdateAssestTypeDto) -> Result<ResEntryAssetTypeDto, UsecaseError>;
    async fn delete_asset_type(&self, user_id: Uuid ,asset_type_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_asset_types(&self, user_id: Uuid) -> Result<ResListAssestTypeDto, UsecaseError>;
}