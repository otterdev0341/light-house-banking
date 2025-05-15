use uuid::Uuid;

use crate::{domain::dto::current_sheet_dto::{ResCurrentSheetDto, ResListCurrentSheetDto}, soc::soc_usercase::UsecaseError};




#[async_trait::async_trait]
pub trait CurrentSheetUsecase {
    async fn get_current_sheet_by_id(&self, user_id: Uuid, curret_sheet_id: Uuid) -> Result<Option<ResCurrentSheetDto>, UsecaseError>;
    async fn get_all_current_sheets_by_user(&self, user_id: Uuid) -> Result<ResListCurrentSheetDto, UsecaseError>;
    async fn get_all_current_sheets_by_asset_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<ResListCurrentSheetDto, UsecaseError>;
}