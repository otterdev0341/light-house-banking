use uuid::Uuid;

use crate::{domain::entities::current_sheet, soc::soc_repository::RepositoryError};




#[async_trait::async_trait]
#[mockall::automock]
pub trait BalanceRepositoryBase {

    async fn get_current_sheet_by_asset_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<Option<current_sheet::Model>, RepositoryError>;
    async fn get_all_current_sheets_by_user(&self, user_id: Uuid) -> Result<Vec<current_sheet::Model>, RepositoryError>;
    async fn create_current_sheet(&self, user_id: Uuid, asset_id: Uuid, initial_balance: f64) -> Result<current_sheet::Model, RepositoryError>;
    async fn update_current_sheet(&self, user_id: Uuid, asset_id: Uuid, balance: Option<f64>) -> Result<current_sheet::Model, RepositoryError>;
    async fn delete_current_sheet_by_asset_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
#[mockall::automock]
pub trait BalanceRepositoryUtill {
    async fn get_all_current_sheets_by_asset_type_id(&self, user_id: Uuid, asset_type_id: Uuid) -> Result<Vec<current_sheet::Model>, RepositoryError>;
    async fn get_all_current_sheets_by_asset_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<Vec<current_sheet::Model>, RepositoryError>;
}