use uuid::Uuid;

use crate::{domain::dto::asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto, ResEntryAssetDto, ResListAssetDto}, soc::soc_usercase::UsecaseError};






#[async_trait::async_trait]
pub trait AssetUsecase {
    async fn create_asset(&self, user_id: Uuid, asset_dto: ReqCreateAssetDto) -> Result<ResEntryAssetDto, UsecaseError>;
    async fn get_asset(&self, user_id: Uuid , asset_id: Uuid) -> Result<Option<ResEntryAssetDto>, UsecaseError>;
    async fn update_asset(&self, user_id: Uuid,  asset_id: Uuid, asset_dto: ReqUpdateAssetDto) -> Result<ResEntryAssetDto, UsecaseError>;
    async fn delete_asset(&self, user_id: Uuid , asset_id: Uuid) -> Result<(), UsecaseError>;
    async fn get_all_asset(&self, user_id: Uuid) -> Result<ResListAssetDto, UsecaseError>;
}