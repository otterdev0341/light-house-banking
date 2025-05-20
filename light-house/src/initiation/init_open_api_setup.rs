use utoipa::OpenApi;

use crate::{configuration::api_doc_config::ApiConfig, swagger_ui::{asset_api::AssetApi, asset_type_api::AssetTypeApi, auth_api::AuthApi, contact_api::ContactApi, contact_type_api::ContactTypeApi, current_sheet_api::CurrentSheetApi, expense_api::ExpenseApi, expense_type::ExpenseTypeApi, income_api::IncomeApi, transaction_type_api::TransactionTypeApi, user_api::UserApi}};




pub fn init_open_api_setup() -> utoipa::openapi::OpenApi {
    let register: Vec<utoipa::openapi::OpenApi> = vec![
        ApiConfig::openapi(),
        AuthApi::openapi(),
        UserApi::openapi(),
        AssetTypeApi::openapi(),
        AssetApi::openapi(),
        ContactTypeApi::openapi(),
        ContactApi::openapi(),
        ExpenseTypeApi::openapi(),
        ExpenseApi::openapi(),
        IncomeApi::openapi(),
        TransactionTypeApi::openapi(),
        CurrentSheetApi::openapi(),
    
    ];

    let mut all_api = register.into_iter();
    let mut mergetd_api = all_api.next().unwrap();

    for api in all_api {
        mergetd_api.merge(api);
    }

    mergetd_api
}
