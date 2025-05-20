use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::{transaction::income_usecase::IncomeUseCase, wrapper::income_wrapper::IncomeRepositoryComposite}, usecase_req_impl::transaction_usecase::RecordIncomeUsecase}, domain::dto::transaction_dto::{ReqCreateIncomeDto, ReqUpdateIncomeDto, ResEntryIncomeDto, ResListIncomeDto}, infrastructure::{database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, contact_repo::ContactRepositoryImpl, transaction_type_repo::TransactionTypeRepositoryImpl}, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn income_routes() -> Vec<Route> {
    routes![
        create_income,
        get_income,
        update_income,
        delete_income,
        get_all_income
    ]
}





#[utoipa::path(
    post,
    path = "/income",
    summary = "Create a new income record",
    description = "Create a new income",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateIncomeDto,
    responses(
        (status = 201, description = "Income record created successfully", body = ResEntryIncomeDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Income"]
)]
#[post("/", data = "<dto>")]
async fn create_income(
    user: AuthenticatedUser,
    dto: Json<ReqCreateIncomeDto>,
    income_usecase: &State<Arc<IncomeUseCase<IncomeRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments

) -> OtterResponse<ResEntryIncomeDto> {
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    log::info!("Creating income with amount: {}", dto.amount);
    match income_usecase.create_income(user.id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}







#[utoipa::path(
    get,
    path = "/income/{transction_id}",
    summary = "Get income by ID",
    description = "Get income by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("transction_id" = String, description = "The ID of the income to retrieve")
    ),
    responses(
        (status = 200, description = "Income retrieved successfully", body = ResEntryIncomeDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Income not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Income"]
)]
#[get("/<transction_id>")]
async fn get_income(
    user: AuthenticatedUser,
    transction_id: Uuid,
    income_usecase: &State<Arc<IncomeUseCase<IncomeRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments

) -> OtterResponse<ResEntryIncomeDto> {
    // field empty Bad request
    if transction_id.is_nil() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", "transaction id is empty"))
        );
    }
    match income_usecase.get_income(user.id, transction_id).await {
        Ok(res) => {
            match res {
                Some(income) => Ok(SuccessResponse(Status::Ok, income)),
                None => Err(ErrorResponse(Status::NotFound, "Income not found".to_string())),
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
    path = "/income/{income_id}",
    summary = "Update an existing income",
    description = "Update an existing income",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("income_id" = String, description = "The ID of the income to update")
    ),
    request_body = ReqUpdateIncomeDto,
    responses(
        (status = 200, description = "Income updated successfully", body = ResEntryIncomeDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Income not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Income"]
)]
#[put("/<income_id>", data = "<dto>")]
pub async fn update_income(
    user: AuthenticatedUser,
    income_id: Uuid,
    dto: Json<ReqUpdateIncomeDto>,
    income_usecase: &State<Arc<IncomeUseCase<IncomeRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments

) -> OtterResponse<ResEntryIncomeDto> {


    if income_id.is_nil() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", "transaction id is empty"))
        );
    }

    match income_usecase.update_income(user.id, income_id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}



#[utoipa::path(
    delete,
    path = "/income/{income_id}",
    summary = "Delete an income by ID",
    description = "Delete an income by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("income_id" = String, description = "The ID of the income to delete")
    ),
    responses(
        (status = 200, description = "Income deleted successfully", body = String),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Income not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Income"]
)]
#[delete("/<income_id>")]
pub async fn delete_income(
    user: AuthenticatedUser,
    income_id: Uuid,
    income_usecase: &State<Arc<IncomeUseCase<IncomeRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments

) -> OtterResponse<String> {
    if income_id.is_nil() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", "transaction id is empty"))
        );
    }
    match income_usecase.delete_income(user.id, income_id).await {
        Ok(_) => Ok(SuccessResponse(Status::Ok, format!("Income with ID {} deleted successfully", income_id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}








#[utoipa::path(
    get,
    path = "/income",
    summary = "Get all income",
    description = "Get all income",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Income retrieved successfully", body = ResListIncomeDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Income"]
)]
#[get("/")]
pub async fn get_all_income(
    user: AuthenticatedUser,
    income_usecase: &State<Arc<IncomeUseCase<IncomeRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments

) -> OtterResponse<ResListIncomeDto> {
    match income_usecase.get_all_income(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}