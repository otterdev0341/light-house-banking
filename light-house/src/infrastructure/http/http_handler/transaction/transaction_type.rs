use std::sync::Arc;

use rocket::{get, http::Status, routes, Route, State};

use crate::{application::{usecase::transaction::transaction_type_usecase::TransactionTypeUseCase, usecase_req_impl::transaction_type_usecase::TransactionTypeUsecase}, domain::dto::transaction_type_dto::ResListTransactionTypeDto, infrastructure::{database::mysql::impl_repository::transaction_type_repo::TransactionTypeRepositoryImpl, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn transaction_type_routes() -> Vec<Route> {
    routes![
        fetch_all_transaction_type,
        
    ]
}



#[utoipa::path(
    get,
    path = "/transaction-type",
    summary = "Fetch all transaction type",
    description = "Fetch all transaction type",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Fetch all transaction type", body = ResListTransactionTypeDto),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Transaction Type"]
)]
#[get("/")]
pub async fn fetch_all_transaction_type(
    user: AuthenticatedUser,
    transaction_type_usecase: &State<Arc<TransactionTypeUseCase<TransactionTypeRepositoryImpl>>>, // Provide all generic arguments
) -> OtterResponse<ResListTransactionTypeDto> {
    match transaction_type_usecase.get_all_transaction_type(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => Err(ErrorResponse(Status::InternalServerError, err.to_string())),
    }
}