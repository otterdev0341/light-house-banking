use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::expense_type_usecase::ExpenseTypeUseCase, usecase_req_impl::expense_type_usecase::ExpenseTypeUsecase}, domain::dto::expense_type_dto::{ReqCreateExpenseTypeDto, ReqUpdateExpenseTypeDto, ResEntryExpenseTypeDto, ResListExpenseTypeDto}, infrastructure::{database::mysql::impl_repository::expense_type_repos::ExpenseTypeRepositoryImpl, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn expense_type_routes() -> Vec<Route> {
    routes![
        create_expense_type,
        view_expense_type_by_id,
        view_all_expense_types,
        delete_expense_type_by_id,
        update_expense_type
    ]
}





#[utoipa::path(
    post,
    path = "/expense-type",
    summary = "Create a new expense type",
    description = "Create a new expense type",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateExpenseTypeDto,
    responses(
        (status = 201, description = "Expense type created successfully", body = ResEntryExpenseTypeDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense Type"]
)]
#[post("/", data = "<dto>")]
pub async fn create_expense_type(
    user: AuthenticatedUser,
    dto: Json<ReqCreateExpenseTypeDto>,
    expense_type_usecase: &State<Arc<ExpenseTypeUseCase<ExpenseTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryExpenseTypeDto> {
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match expense_type_usecase.create_expense_type(user.id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}




#[utoipa::path(
    get,
    path = "/expense-type/{expense_type_id}",
    summary = "Get an expense type by ID",
    description = "Get an expense type by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("expense_type_id" = String, description = "The ID of the expense type"),
    ),
    responses(
        (status = 200, description = "Expense type found", body = ResEntryExpenseTypeDto),
        (status = 404, description = "Expense type not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense Type"]
)]
#[get("/<expense_type_id>")]
pub async fn view_expense_type_by_id(
    user: AuthenticatedUser,
    expense_type_id: Uuid,
    expense_type_usecase: &State<Arc<ExpenseTypeUseCase<ExpenseTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryExpenseTypeDto> {
    
    if expense_type_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }
    
    
    match expense_type_usecase.get_expense_type(user.id, expense_type_id).await {
        Ok(res) => {
            match res {
                Some(expense_type) => Ok(SuccessResponse(Status::Ok, expense_type)),
                None => Err(ErrorResponse(Status::NotFound, "Expense type not found".to_string())),
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
    path = "/expense-type",
    summary = "Get all expense types",
    description = "Get all expense types",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Expense types found", body = ResListExpenseTypeDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense Type"]
)]
#[get("/")]
pub async fn view_all_expense_types(
    user: AuthenticatedUser,
    expense_type_usecase: &State<Arc<ExpenseTypeUseCase<ExpenseTypeRepositoryImpl>>>,
) -> OtterResponse<ResListExpenseTypeDto> {
    match expense_type_usecase.get_all_expense_type(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}





#[utoipa::path(
    delete,
    path = "/expense-type/{expense_type_id}",
    summary = "Delete an expense type by ID",
    description = "Delete an expense type by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("expense_type_id" = String, description = "The ID of the expense type"),
    ),
    responses(
        (status = 200, description = "Expense type deleted successfully", body = String),
        (status = 404, description = "Expense type not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense Type"]
)]
#[delete("/<expense_type_id>")]
pub async fn delete_expense_type_by_id(
    user: AuthenticatedUser,
    expense_type_id: Uuid,
    expense_type_usecase: &State<Arc<ExpenseTypeUseCase<ExpenseTypeRepositoryImpl>>>,
) -> OtterResponse<String> {
    if expense_type_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }
    
    match expense_type_usecase.delete_expense_type(user.id, expense_type_id).await {
        Ok(_) => Ok(SuccessResponse(Status::Ok, format!("Expense type with ID {} deleted successfully", expense_type_id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}



#[utoipa::path(
    put,
    path = "/expense-type/{expense_type_id}",
    summary = "Update an expense type by ID",
    description = "Update an expense type by ID",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqUpdateExpenseTypeDto,
    params(
        ("expense_type_id" = String, description = "The ID of the expense type"),
    ),
    responses(
        (status = 200, description = "Expense type updated successfully", body = ResEntryExpenseTypeDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 404, description = "Expense type not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense Type"]
)]
#[put("/<expense_type_id>", data = "<dto>")]
pub async fn update_expense_type(
    user: AuthenticatedUser,
    expense_type_id: Uuid,
    dto: Json<ReqUpdateExpenseTypeDto>,
    expense_type_usecase: &State<Arc<ExpenseTypeUseCase<ExpenseTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryExpenseTypeDto> {

    match expense_type_usecase.update_expense_type(user.id, expense_type_id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}