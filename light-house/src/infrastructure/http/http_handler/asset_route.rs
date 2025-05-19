use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::asset_usecase::AssetUseCase, usecase_req_impl::asset_usecase::AssetUsecase}, domain::dto::asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto, ResEntryAssetDto, ResListAssetDto}, infrastructure::{database::mysql::impl_repository::asset_repo::AssetRepositoryImpl, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};







pub fn asset_routes() -> Vec<Route> {
    routes![
       create_asset_type,
        view_asset_by_id,
        view_all_asset,
        delete_asset_by_id,
        update_asset,
    ]
}


#[utoipa::path(
    post,
    path = "/asset",
    summary = "Create a new asset",
    description = "Create a new asset",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateAssetDto,
    responses(
        (status = 201, description = "Asset created successfully", body = ResEntryAssetDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Asset"]
)]
#[post("/", data = "<asset_dto>")]
pub async fn create_asset_type(
    user: AuthenticatedUser,
    asset_dto: Json<ReqCreateAssetDto>,
    asset_usecase: &State<Arc<AssetUseCase<AssetRepositoryImpl>>>,
) -> OtterResponse<ResEntryAssetDto> 
{
    //field empty Bad request
    if let Err(errors) = asset_dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match asset_usecase.create_asset(user.id, asset_dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError,err.to_string());
            Err(error_response)
        }
    }
}







#[utoipa::path(
    get,
    path = "/asset/{asset_id}",
    summary = "Get asset by ID",
    description = "Get asset by ID",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Asset found", body = ResEntryAssetDto),
        (status = 404, description = "Asset not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Asset"]
)]
#[get("/<asset_id>")]
pub async fn view_asset_by_id(
    user: AuthenticatedUser,
    asset_id: Uuid,
    asset_usecase: &State<Arc<AssetUseCase<AssetRepositoryImpl>>>,
) -> OtterResponse<ResEntryAssetDto> {

    if asset_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid asset type ID".to_string()));
    }

    match asset_usecase.get_asset(user.id, asset_id).await {
        Ok(res) => {
            match res {
                Some(asset) => Ok(SuccessResponse(Status::Ok, asset)),
                None => Err(ErrorResponse(Status::NotFound, "Asset not found".to_string())),
            }
        },
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}




#[utoipa::path(
    get,
    path = "/asset",
    summary = "Get all assets",
    description = "Get all assets",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Assets found", body = ResListAssetDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Asset"]
)]
#[get("/")]
pub async fn view_all_asset(
    user: AuthenticatedUser,
    asset_usecase: &State<Arc<AssetUseCase<AssetRepositoryImpl>>>,
) -> OtterResponse<ResListAssetDto> {
    match asset_usecase.get_all_asset(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}






#[utoipa::path(
    delete,
    path = "/asset/{asset_id}",
    summary = "Delete asset by ID",
    description = "Delete asset by ID",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Asset deleted successfully", body = String),
        (status = 404, description = "Asset not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Asset"]
)]
#[delete("/<asset_id>")]
pub async fn delete_asset_by_id(
    user: AuthenticatedUser,
    asset_id: Uuid,
    asset_usecase: &State<Arc<AssetUseCase<AssetRepositoryImpl>>>,
) -> OtterResponse<String> {
    
    if asset_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid asset ID".to_string()));
    }

    match asset_usecase.delete_asset(user.id, asset_id).await {
        Ok(_) => Ok(SuccessResponse(Status::Ok, format!("Asset with ID {} deleted successfully", asset_id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}






#[utoipa::path(
    put,
    path = "/asset/{asset_id}",
    summary = "Update asset by ID",
    description = "Update asset by ID",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqUpdateAssetDto,
    responses(
        (status = 200, description = "Asset updated successfully", body = ResEntryAssetDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 404, description = "Asset not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Asset"]
)]
#[put("/<asset_id>", data = "<asset_dto>")]
pub async fn update_asset(
    user: AuthenticatedUser,
    asset_id: Uuid,
    asset_dto: Json<ReqUpdateAssetDto>,
    asset_usecase: &State<Arc<AssetUseCase<AssetRepositoryImpl>>>,
) -> OtterResponse<ResEntryAssetDto> {
    if asset_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid asset ID".to_string()));
    }
    match asset_usecase.update_asset(user.id, asset_id, asset_dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}