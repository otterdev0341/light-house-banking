use std::sync::Arc;

use rocket::{get, http::Status, routes, Route, State};
use uuid::Uuid;

use crate::{application::{usecase::current_sheet_usecase::CurrentUseCase, usecase_req_impl::current_sheet_usecase::CurrentSheetUsecase}, domain::dto::current_sheet_dto::{ResCurrentSheetDto, ResListCurrentSheetDto}, infrastructure::{database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, balance_repo::BalanceRepositoryImpl}, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn current_sheet_routes() -> Vec<Route> {
    routes![
        fetch_current_sheet_by_current_sheet_by_id,
        fetch_all_current_sheets_by_user_id,
        fetch_all_current_sheets_by_asset_id
    ]
}






#[utoipa::path(
    get,
    path = "/current-sheet/{current_sheet_id}",
    summary = "Fetch current sheet by ID",
    description = "Fetch current sheet by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("current_sheet_id" = String, description = "The ID of the current sheet"),
    ),
    responses(
        (status = 200, description = "Current sheet fetched successfully", body = ResCurrentSheetDto),
        (status = 400, description = "Invalid current sheet ID", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Current sheet not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Current Sheet"]
)]
#[get("/<current_sheet_id>")]
pub async fn fetch_current_sheet_by_current_sheet_by_id(
    user: AuthenticatedUser,
    current_sheet_id: Uuid,
    current_sheet_usecase: &State<Arc<CurrentUseCase<BalanceRepositoryImpl, AssetRepositoryImpl>>>,
) -> OtterResponse<ResCurrentSheetDto> {
    
    if current_sheet_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid current sheet ID".to_string()));
    }
    
    match current_sheet_usecase
        .get_current_sheet_by_id(user.id, current_sheet_id)
        .await
    {
        Ok(Some(res)) => Ok(SuccessResponse(Status::Ok, res)),
        Ok(None) => Err(ErrorResponse(Status::NotFound, "Current sheet not found".to_string())),
        Err(err) => Err(ErrorResponse(Status::InternalServerError, err.to_string())),
    }
}






#[utoipa::path(
    get,
    path = "/current-sheet",
    summary = "Fetch all current sheets by user ID",
    description = "Fetch all current sheets by user ID",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Current sheets fetched successfully", body = ResListCurrentSheetDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Current sheets not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Current Sheet"]
)]
#[get("/")]
pub async fn fetch_all_current_sheets_by_user_id(
    user: AuthenticatedUser,
    current_sheet_usecase: &State<Arc<CurrentUseCase<BalanceRepositoryImpl, AssetRepositoryImpl>>>, // Ensure generic arguments match

) -> OtterResponse<ResListCurrentSheetDto> {
    
    match current_sheet_usecase
        .get_all_current_sheets_by_user(user.id)
        .await
    {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => Err(ErrorResponse(Status::InternalServerError, err.to_string())),
    }
}




#[utoipa::path(
    get,
    path = "/current-sheet/{asset_id}/asset",
    summary = "Fetch all current sheets by asset ID",
    description = "Fetch all current sheets by asset ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("asset_id" = String, description = "The ID of the asset"),
    ),
    responses(
        (status = 200, description = "Current sheets fetched successfully", body = ResListCurrentSheetDto),
        (status = 400, description = "Invalid asset ID", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Current sheets not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Current Sheet"]
)]
#[get("/<asset_id>/asset")]
pub async fn fetch_all_current_sheets_by_asset_id(
    user: AuthenticatedUser,
    asset_id: Uuid,
    current_sheet_usecase: &State<Arc<CurrentUseCase<BalanceRepositoryImpl, AssetRepositoryImpl>>>,
) -> OtterResponse<ResListCurrentSheetDto> {
    
    if asset_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid asset ID".to_string()));
    }
    
    match current_sheet_usecase
        .get_all_current_sheets_by_asset_id(user.id, asset_id)
        .await
    {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => Err(ErrorResponse(Status::InternalServerError, err.to_string())),
    }
}