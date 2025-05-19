use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::expense_usecase::ExpenseUseCase, usecase_req_impl::expense_usecase::ExpenseUsecase}, domain::dto::expense_dto::{ReqCreateExpenseDto, ReqUpdateExpenseDto, ResEntryExpenseDto, ResListExpenseDto}, infrastructure::{database::mysql::impl_repository::expense_repo::ExpenseRepositoryImpl, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn expense_routes() -> Vec<Route> {
    routes![
        create_expense,
        view_expense_by_id,
        view_all_expenses,
        delete_expense_by_id,
        update_expense_by_id
    ]
}





#[utoipa::path(
    post,
    path = "/expense",
    summary = "Create a new expense",
    description = "Create a new expense",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateExpenseDto,
    responses(
        (status = 201, description = "Expense created successfully", body = ResEntryExpenseDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense"]
)]
#[post("/", data = "<dto>")]
async fn create_expense(
    user: AuthenticatedUser,
    dto: Json<ReqCreateExpenseDto>,
    expense_usecase: &State<Arc<ExpenseUseCase<ExpenseRepositoryImpl>>>,
) -> OtterResponse<ResEntryExpenseDto> {
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match expense_usecase.create_expense(user.id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}



#[utoipa::path(
    get,
    path = "/expense/{expense_id}",
    summary = "Get an expense by ID",
    description = "Get an expense by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("expense_id" = String, description = "The ID of the expense to retrieve")
    ),
    responses(
        (status = 200, description = "Expense retrieved successfully", body = ResEntryExpenseDto),
        (status = 404, description = "Expense not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense"]
)]
#[get("/<expense_id>")]
pub async fn view_expense_by_id(
    user: AuthenticatedUser,
    expense_id: Uuid,
    expense_usecase: &State<Arc<ExpenseUseCase<ExpenseRepositoryImpl>>>,
) -> OtterResponse<ResEntryExpenseDto> {

    if expense_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }

    match expense_usecase.get_expense(user.id, expense_id).await {
        Ok(res) => {
            match res {
                Some(expense) => Ok(SuccessResponse(Status::Ok, expense)),
                None => Err(ErrorResponse(Status::NotFound, "Expense not found".to_string())),
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
    path = "/expense",
    summary = "Get all expenses",
    description = "Get all expenses",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Expenses retrieved successfully", body = ResListExpenseDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense"]
)]
#[get("/")]
pub async fn view_all_expenses(
    user: AuthenticatedUser,
    expense_usecase: &State<Arc<ExpenseUseCase<ExpenseRepositoryImpl>>>,
) -> OtterResponse<ResListExpenseDto> {
    match expense_usecase.get_all_expense(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}




#[utoipa::path(
    delete,
    path = "/expense/{expense_id}",
    summary = "Delete an expense by ID",
    description = "Delete an expense by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("expense_id" = String, description = "The ID of the expense to delete")
    ),
    responses(
        (status = 200, description = "Expense deleted successfully", body = String),
        (status = 404, description = "Expense not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense"]
)]
#[delete("/<expense_id>")]
pub async fn delete_expense_by_id(
    user: AuthenticatedUser,
    expense_id: Uuid,
    expense_usecase: &State<Arc<ExpenseUseCase<ExpenseRepositoryImpl>>>,
) -> OtterResponse<String> {
    
    if expense_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }

    match expense_usecase.delete_expense(user.id, expense_id).await {
        Ok(_) => Ok(SuccessResponse(Status::Ok, format!("Expense with ID {} deleted successfully", expense_id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}





#[utoipa::path(
    put,
    path = "/expense/{expense_id}",
    summary = "Update an expense by ID",
    description = "Update an expense by ID",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqUpdateExpenseDto,
    params(
        ("expense_id" = String, description = "The ID of the expense to update")
    ),
    responses(
        (status = 200, description = "Expense updated successfully", body = ResEntryExpenseDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 404, description = "Expense not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Expense"]
)]
#[put("/<expense_id>", data = "<dto>")]
pub async fn update_expense_by_id(
    user: AuthenticatedUser,
    expense_id: Uuid,
    dto: Json<ReqUpdateExpenseDto>,
    expense_usecase: &State<Arc<ExpenseUseCase<ExpenseRepositoryImpl>>>,
) -> OtterResponse<ResEntryExpenseDto> {
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match expense_usecase.update_expense(user.id, expense_id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}