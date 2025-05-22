use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::{transaction::transfer_usecase::TransferUseCase, wrapper::transfer_wrapper::TransferRepositoryComposite}, usecase_req_impl::transaction_usecase::TransferUsecase}, domain::dto::transaction_dto::{ReqCreateTransferDto, ReqUpdateTransferDto, ResEntryTransferDto, ResListTransferDto}, infrastructure::{database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, contact_repo::ContactRepositoryImpl, transaction_type_repo::TransactionTypeRepositoryImpl}, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn transfer_routes() -> Vec<Route> {
    routes![
        create_tranfer,
        get_transfer,
        update_transfer,
        delete_transfer,
        get_all_transfer
    ]
}





#[utoipa::path(
    post,
    path = "/transfer",
    summary = "Create a new transfer record",
    description = "Create a new transfer",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateTransferDto,
    responses(
        (status = 201, description = "Transfer record created successfully", body = ResEntryTransferDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Transfer"]
)]
#[post("/", data = "<dto>")]
async fn create_tranfer(
    user: AuthenticatedUser,
    dto: Json<ReqCreateTransferDto>,
    transfer_usecase: &State<Arc<TransferUseCase<TransferRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResEntryTransferDto> {
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    log::info!("Creating transfer with amount: {}", dto.amount);
    match transfer_usecase.create_transfer(user.id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}




#[utoipa::path(
    get,
    path = "/transfer/{id}",
    summary = "Get a transfer record by ID",
    description = "Retrieve a transfer record by its ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = String, description = "ID of the transfer record"),
    ),
    responses(
        (status = 200, description = "Transfer record found", body = ResEntryTransferDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Transfer not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Transfer"]
)]
#[get("/<id>")]
async fn get_transfer(
    user: AuthenticatedUser,
    id: Uuid,
    transfer_usecase: &State<Arc<TransferUseCase<TransferRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResEntryTransferDto> {
    // field empty Bad request
    if id.is_nil() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", "Id is empty"))
        );
    }

    match transfer_usecase.get_transfer(user.id, id).await {
        Ok(res) => {
            match res {
                Some(transfer) => Ok(SuccessResponse(Status::Ok, transfer)),
                None => Err(ErrorResponse(Status::NotFound, "Transfer not found".to_string())),
            }
        },
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
        
    }
}









#[utoipa::path(
    put,
    path = "/transfer/{id}",
    summary = "Update a transfer record",
    description = "Update a transfer record by ID",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqUpdateTransferDto,
    params(
        ("id" = String, description = "ID of the transfer record to update"),
    ),
    responses(
        (status = 200, description = "Transfer record updated successfully", body = ResEntryTransferDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Transfer not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Transfer"]
)]
#[put("/<id>", data = "<dto>")]
async fn update_transfer(
    user: AuthenticatedUser,
    id: Uuid,
    dto: Json<ReqUpdateTransferDto>,
    transfer_usecase: &State<Arc<TransferUseCase<TransferRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResEntryTransferDto> {
    // field empty Bad request
    if id.is_nil() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", "Id is empty"))
        );
    }

    match transfer_usecase.update_transfer(user.id, id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}







#[utoipa::path(
    delete,
    path = "/transfer/{id}",
    summary = "Delete a transfer record",
    description = "Delete a transfer record by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = String, description = "ID of the transfer record to delete"),
    ),
    responses(
        (status = 200, description = "Transfer record deleted successfully", body = String),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Transfer not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Transfer"]
)]
#[delete("/<id>")]
async fn delete_transfer(
    user: AuthenticatedUser,
    id: Uuid,
    transfer_usecase: &State<Arc<TransferUseCase<TransferRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<String> {
    // field empty Bad request
    if id.is_nil() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", "Id is empty"))
        );
    }

    match transfer_usecase.delete_transfer(user.id, id).await {
        Ok(_) => Ok(SuccessResponse(Status::Ok, format!("Transfer with ID {} deleted successfully", id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}




#[utoipa::path(
    get,
    path = "/transfer",
    summary = "Get all transfer records",
    description = "Retrieve all transfer records",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Transfer records found", body = ResListTransferDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Transfer"]
)]
#[get("/")]
async fn get_all_transfer(
    user: AuthenticatedUser,
    transfer_usecase: &State<Arc<TransferUseCase<TransferRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResListTransferDto> {
    match transfer_usecase.get_all_transfer(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}