use std::sync::Arc;

use chrono::format;
use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::{transaction::payment_usecase::PaymentUseCase, wrapper::payment_wrapper::PaymentRepositoryComposite}, usecase_req_impl::transaction_usecase::RecordPaymentUsecase}, domain::dto::transaction_dto::{ReqCreatePaymentDto, ReqUpdatePaymentDto, ResEntryPaymentDto, ResListPaymentDto}, infrastructure::{database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, contact_repo::ContactRepositoryImpl, transaction_type_repo::TransactionTypeRepositoryImpl}, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn payment_routes() -> Vec<Route> {
    routes![
        create_payment,
        get_payment,
        update_payment,
        delete_payment,
        get_all_payments
    ]
}



#[utoipa::path(
    post,
    path = "/payment",
    summary = "Create a new payment record",
    description = "Create a new payment",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreatePaymentDto,
    responses(
        (status = 201, description = "Payment record created successfully", body = ResEntryPaymentDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Payment"]
)]
#[post("/", data = "<dto>")]
async fn create_payment(
    user: AuthenticatedUser,
    dto: Json<ReqCreatePaymentDto>,
    payment_usecase: &State<Arc<PaymentUseCase<PaymentRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResEntryPaymentDto> {
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    log::info!("Creating payment with amount: {}", dto.amount);
    match payment_usecase.create_payment(user.id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}







#[utoipa::path(
    get,
    path = "/payment/{payment_id}",
    summary = "Get a payment record by ID",
    description = "Get a payment record by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("payment_id" = String, description = "The ID of the payment to retrieve")
    ),
    responses(
        (status = 200, description = "Payment record retrieved successfully", body = ResEntryPaymentDto),
        (status = 400, description = "Invalid payment ID", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Payment not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Payment"]
)]
#[get("/<payment_id>")]
async fn get_payment(
    user: AuthenticatedUser,
    payment_id: Uuid,
    payment_usecase: &State<Arc<PaymentUseCase<PaymentRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResEntryPaymentDto> {
    
    if payment_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid payment ID".to_string()));
    }

    log::info!("Getting payment with ID: {}", payment_id);

    match payment_usecase.get_payment(user.id, payment_id).await {
        Ok(res) => {
            match res {
                Some(payment) => Ok(SuccessResponse(Status::Ok, payment)),
                None => Err(ErrorResponse(Status::NotFound, "Payment not found".to_string())),
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
    path = "/payment/{payment_id}",
    summary = "Update a payment record by ID",
    description = "Update a payment record by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("payment_id" = String, description = "The ID of the payment to update")
    ),
    request_body = ReqUpdatePaymentDto,
    responses(
        (status = 200, description = "Payment record updated successfully", body = ResEntryPaymentDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Payment not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Payment"]
)]
#[put("/<payment_id>", data = "<dto>")]
async fn update_payment(
    user: AuthenticatedUser,
    payment_id: Uuid,
    dto: Json<ReqUpdatePaymentDto>,
    payment_usecase: &State<Arc<PaymentUseCase<PaymentRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResEntryPaymentDto> {
    
    if payment_id.is_nil() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", "payment id is empty"))
        );
    }

    log::info!("Updating payment with ID: {}", payment_id);
    match payment_usecase.update_payment(user.id, payment_id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}











#[utoipa::path(
    delete,
    path = "/payment/{payment_id}",
    summary = "Delete a payment record by ID",
    description = "Delete a payment record by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("payment_id" = String, description = "The ID of the payment to delete")
    ),
    responses(
        (status = 204, description = "Payment record deleted successfully"),
        (status = 400, description = "Invalid payment ID", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Payment not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Payment"]
)]
#[delete("/<payment_id>")]
async fn delete_payment(
    user: AuthenticatedUser,
    payment_id: Uuid,
    payment_usecase: &State<Arc<PaymentUseCase<PaymentRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<String> {
    
    if payment_id.is_nil() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", "payment id is empty"))
        );
    }

    log::info!("Deleting payment with ID: {}", payment_id);
    match payment_usecase.delete_payment(user.id, payment_id).await {
        Ok(_) => Ok(SuccessResponse(Status::NoContent, format!("Payment with ID: {} deleted successfully", payment_id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}








#[utoipa::path(
    get,
    path = "/payment",
    summary = "Get all payments for a user",
    description = "Get all payments for a user",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of payments retrieved successfully", body = ResListPaymentDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Payment"]
)]
#[get("/")]
async fn get_all_payments(
    user: AuthenticatedUser,
    payment_usecase: &State<Arc<PaymentUseCase<PaymentRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResListPaymentDto> {
    log::info!("Getting all payments for user ID: {}", user.id);
    match payment_usecase.get_all_payment(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}