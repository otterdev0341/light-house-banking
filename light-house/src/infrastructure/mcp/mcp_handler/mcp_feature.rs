use std::sync::Arc;

use rocket::{get, http::Status, routes, Route, State};

use crate::{application::{usecase::{asset_type_usecase::AssetTypeUseCase, asset_usecase::AssetUseCase, contact_type_usecase::ContactTypeUseCase, contact_usecase::ContactUseCase, expense_type_usecase::ExpenseTypeUseCase, expense_usecase::ExpenseUseCase, transaction::{income_usecase::IncomeUseCase, payment_usecase::PaymentUseCase, transfer_usecase::TransferUseCase}, user_usecase::UserUseCase, wrapper::{income_wrapper::IncomeRepositoryComposite, payment_wrapper::PaymentRepositoryComposite, transfer_wrapper::TransferRepositoryComposite, user_wrapper::UserRepositoryComposite}}, usecase_req_impl::{asset_type_usecase::AssetTypeUsecase, asset_usecase::AssetUsecase, contact_type_usecase::ContactTypeUsecase, contact_usecase::ContactUsecase, expense_type_usecase::ExpenseTypeUsecase, expense_usecase::ExpenseUsecase, transaction_usecase::{RecordIncomeUsecase, RecordPaymentUsecase, TransferUsecase}, user_usecase::UserUsecase}}, domain::dto::{assest_type_dto::ResListAssestTypeDto, asset_dto::ResListAssetDto, auth_dto::ResMeDto, contact_dto::ResListContactDto, contact_type_dto::ResListContactTypeDto, expense_dto::ResListExpenseDto, expense_type_dto::ResListExpenseTypeDto, transaction_dto::{ResListIncomeDto, ResListPaymentDto, ResListTransferDto}}, infrastructure::{database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, asset_type_repo::AssetTypeRepositoryImpl, contact_repo::ContactRepositoryImpl, contact_type_repo::ContactTypeRepositoryImpl, expense_repo::ExpenseRepositoryImpl, expense_type_repos::ExpenseTypeRepositoryImpl, transaction_type_repo::TransactionTypeRepositoryImpl}, http::{faring::mcp_auth::McpAuthenticateUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};


// init_route
pub fn mcp_routes() -> Vec<Route>{
    routes![
        // >>> user
        mcp_resme,

        // >>> transaction
        mcp_get_all_income,
        mcp_get_all_payment,
        mcp_get_all_transfer,

        // >>> type
        mcp_get_all_contact_type,
        mcp_get_all_expense_type,
        mcp_get_all_asset_type,

        // main
        mcp_get_all_contact,
        mcp_get_all_expense,
        mcp_get_all_asset,
    ]
}



// resme
#[get("/resme")]
pub async fn mcp_resme(
    user: McpAuthenticateUser,
    user_usecase: &State<Arc<UserUseCase<UserRepositoryComposite>>>
) -> OtterResponse<ResMeDto> {
    match user_usecase.me(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}



// >>> transaction
#[get("/income")]
pub async fn mcp_get_all_income(
    user: McpAuthenticateUser,
    income_usecase: &State<Arc<IncomeUseCase<IncomeRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>
) -> OtterResponse<ResListIncomeDto> {
    match income_usecase.get_all_income(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}


#[get("/payment")]
pub async fn mcp_get_all_payment(
    user: McpAuthenticateUser,
    payment_usecase: &State<Arc<PaymentUseCase<PaymentRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl, ExpenseRepositoryImpl>>>
) -> OtterResponse<ResListPaymentDto> {
    match payment_usecase.get_all_payment(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}


#[get("/transfer")]
pub async fn mcp_get_all_transfer(
    user: McpAuthenticateUser,
    transfer_usecase: &State<Arc<TransferUseCase<TransferRepositoryComposite, AssetRepositoryImpl, ContactRepositoryImpl, TransactionTypeRepositoryImpl>>>
) -> OtterResponse<ResListTransferDto> {
    match transfer_usecase.get_all_transfer(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}


// >>> type

#[get("/contact-type")]
pub async fn mcp_get_all_contact_type(
    user: McpAuthenticateUser,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ContactTypeRepositoryImpl>>>,
) -> OtterResponse<ResListContactTypeDto>{
    match contact_type_usecase.get_all_contact_type(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}

#[get("/expense-type")]
pub async fn mcp_get_all_expense_type(
    user: McpAuthenticateUser,
    expense_type_usecase: &State<Arc<ExpenseTypeUseCase<ExpenseTypeRepositoryImpl>>>,
) -> OtterResponse<ResListExpenseTypeDto> {
    match expense_type_usecase.get_all_expense_type(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}
// get all asset type
#[get("/asset-type")]
pub async fn mcp_get_all_asset_type(
    user: McpAuthenticateUser,
    asset_type_usecase: &State<Arc<AssetTypeUseCase<AssetTypeRepositoryImpl>>>,
) -> OtterResponse<ResListAssestTypeDto> {
    match asset_type_usecase.get_all_asset_types(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}



// main
// get all contact
#[get("/contact")]
pub async fn mcp_get_all_contact(
    user: McpAuthenticateUser,
    contact_usecase: &State<Arc<ContactUseCase<ContactRepositoryImpl>>>,
) -> OtterResponse<ResListContactDto> {
    match contact_usecase.get_all_contact(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}
// get all expense
#[get("/expense")]
pub async fn mcp_get_all_expense(
    user: McpAuthenticateUser,
    expense_usecase: &State<Arc<ExpenseUseCase<ExpenseRepositoryImpl>>>,
) -> OtterResponse<ResListExpenseDto> {
    match expense_usecase.get_all_expense(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}
// get all asset
#[get("/asset")]
pub async fn mcp_get_all_asset(
    user: McpAuthenticateUser,
    asset_usecase: &State<Arc<AssetUseCase<AssetRepositoryImpl>>>,
) -> OtterResponse<ResListAssetDto> {
    match asset_usecase.get_all_asset(user.user_id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}