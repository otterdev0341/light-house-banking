use std::{ops::Deref, sync::Arc};

use uuid::Uuid;

use crate::{domain::{dto::asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto}, entities::{asset, current_sheet}, req_repository::{asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}, balance_repository::{BalanceRepositoryBase, BalanceRepositoryUtill}}}, infrastructure::database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, balance_repo::BalanceRepositoryImpl}, soc::soc_repository::RepositoryError};






pub struct CurrentSheetRepositoryComposite {
    pub balance_repository: Arc<BalanceRepositoryImpl>,
    pub asset_repository: Arc<AssetRepositoryImpl>,
}


impl CurrentSheetRepositoryComposite {
    pub fn balance_repository(&self) -> &BalanceRepositoryImpl {
        &self.balance_repository
    }

    pub fn asset_repository(&self) -> &AssetRepositoryImpl {
        &self.asset_repository
    }

}

impl Deref for CurrentSheetRepositoryComposite {
    type Target = BalanceRepositoryImpl;

    fn deref(&self) -> &Self::Target {
        &self.balance_repository
    }
}


#[async_trait::async_trait]
impl BalanceRepositoryBase for CurrentSheetRepositoryComposite {
    async fn get_current_sheet_by_asset_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<Option<current_sheet::Model>, RepositoryError>
    {
        self.balance_repository.get_current_sheet_by_asset_id(user_id, asset_id).await
    }
    async fn get_all_current_sheets_by_user(&self, user_id: Uuid) -> Result<Vec<current_sheet::Model>, RepositoryError>
    {
        self.balance_repository.get_all_current_sheets_by_user(user_id).await
    }
    async fn create_current_sheet(&self, user_id: Uuid, asset_id: Uuid, initial_balance: f64) -> Result<current_sheet::Model, RepositoryError>
    {
        self.balance_repository.create_current_sheet(user_id, asset_id, initial_balance).await
    }
    async fn update_current_sheet(&self, user_id: Uuid, asset_id: Uuid, balance: Option<f64>) -> Result<current_sheet::Model, RepositoryError>
    {
        self.balance_repository.update_current_sheet(user_id, asset_id, balance).await
    }
    async fn delete_current_sheet_by_asset_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<(), RepositoryError>
    {
        self.balance_repository.delete_current_sheet_by_asset_id(user_id, asset_id).await
    }
    async fn get_current_sheet_by_id(&self, user_id: Uuid, current_sheet_id: Uuid) -> Result<Option<current_sheet::Model>, RepositoryError>
    {
        self.balance_repository.get_current_sheet_by_id(user_id, current_sheet_id).await
    }
}

#[async_trait::async_trait]
impl BalanceRepositoryUtill for CurrentSheetRepositoryComposite{
    async fn get_all_current_sheets_by_asset_type_id(&self, user_id: Uuid, asset_type_id: Uuid) -> Result<Vec<current_sheet::Model>, RepositoryError>
    {
        self.balance_repository.get_all_current_sheets_by_asset_type_id(user_id, asset_type_id).await
    }
    async fn get_all_current_sheets_by_asset_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<Vec<current_sheet::Model>, RepositoryError>
    {
        self.balance_repository.get_all_current_sheets_by_asset_id(user_id, asset_id).await
    }
}


#[async_trait::async_trait]
impl AssetRepositoryBase for CurrentSheetRepositoryComposite {
    async fn create(&self, user_id: Uuid, dto: ReqCreateAssetDto) -> Result<asset::Model, RepositoryError>
    {
        self.asset_repository.create(user_id, dto).await
    }
    async fn find_by_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<Option<asset::Model>, RepositoryError>
    {
        self.asset_repository.find_by_id(user_id, asset_id).await
    }
    async fn find_all(&self) -> Result<Vec<asset::Model>, RepositoryError>
    {
        self.asset_repository.find_all().await
    }
    async fn update(&self, dto: ReqUpdateAssetDto, user_id: Uuid, asset_id: Uuid) -> Result<asset::Model, RepositoryError>
    {
        self.asset_repository.update(dto, user_id, asset_id).await
    }
    async fn delete(&self,user_id: Uuid, asset_id : Uuid) -> Result<(), RepositoryError>
    {
        self.asset_repository.delete(user_id, asset_id).await
    }
}

#[async_trait::async_trait]
impl AssetRepositoryUtility for CurrentSheetRepositoryComposite{
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset::Model>, RepositoryError>
    {
        self.asset_repository.find_all_by_user_id(user_id).await
    }
}