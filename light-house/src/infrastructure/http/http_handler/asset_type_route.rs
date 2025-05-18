use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::asset_type_usecase::AssetTypeUseCase, usecase_req_impl::asset_type_usecase::AssetTypeUsecase}, domain::dto::assest_type_dto::{ReqCreateAssetTypeDto, ReqUpdateAssestTypeDto, ResEntryAssetTypeDto, ResListAssestTypeDto}, infrastructure::{database::mysql::impl_repository::asset_type_repo::AssetTypeRepositoryImpl, http::{faring::{authentication::AuthenticatedUser, cors::options}, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};




pub fn asset_type_routes() -> Vec<Route> {
    routes![
        create_asset_type,
        view_asset_type_by_id,
        view_all_asset_types,
        delete_asset_type_by_id,
        update_asset_type,
        
    ]
}


#[post("/", data = "<dto>")]
pub async fn create_asset_type(
    user: AuthenticatedUser,
    dto: Json<ReqCreateAssetTypeDto>,
    asset_type_usecase: &State<Arc<AssetTypeUseCase<AssetTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryAssetTypeDto>
{
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match asset_type_usecase.create_asset_type(user.id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}


#[get("/<asset_type_id>")]
pub async fn view_asset_type_by_id(
    user: AuthenticatedUser,
    asset_type_id: Uuid,
    asset_type_usecase: &State<Arc<AssetTypeUseCase<AssetTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryAssetTypeDto>
{
    if asset_type_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid asset type ID".to_string()));
    }
    match asset_type_usecase.get_asset_type(user.id, asset_type_id).await {
        Ok(res) => {
            match res {
                Some(asset_type) => Ok(SuccessResponse(Status::Ok, asset_type)),
                None => Err(ErrorResponse(Status::NotFound, "Asset type not found".to_string())),
            }
        },
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}

#[get("/")]
pub async fn view_all_asset_types(
    user: AuthenticatedUser,
    asset_type_usecase: &State<Arc<AssetTypeUseCase<AssetTypeRepositoryImpl>>>,
) -> OtterResponse<ResListAssestTypeDto>
{
    match asset_type_usecase.get_all_asset_types(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}

#[delete("/<asset_type_id>")]
pub async fn delete_asset_type_by_id(
    user: AuthenticatedUser,
    asset_type_id: Uuid,
    asset_type_usecase: &State<Arc<AssetTypeUseCase<AssetTypeRepositoryImpl>>>,
) -> OtterResponse<String>
{
    match asset_type_usecase.delete_asset_type(user.id, asset_type_id).await {
        Ok(_) => Ok(SuccessResponse(Status::Ok, format!("Asset type with id {} deleted successfully", asset_type_id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}

#[put("/<asset_type_id>", data = "<dto>")]
pub async fn update_asset_type(
    user: AuthenticatedUser,
    asset_type_id: Uuid,
    dto: Json<ReqUpdateAssestTypeDto>,
    asset_type_usecase: &State<Arc<AssetTypeUseCase<AssetTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryAssetTypeDto>
{
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match asset_type_usecase.update_asset_type(user.id, asset_type_id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}