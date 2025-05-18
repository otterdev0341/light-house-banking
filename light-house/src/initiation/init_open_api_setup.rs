use utoipa::OpenApi;

use crate::{configuration::api_doc_config::ApiConfig, swagger_ui::{auth_api::AuthApi, user_api::UserApi}};




pub fn init_open_api_setup() -> utoipa::openapi::OpenApi {
    let register: Vec<utoipa::openapi::OpenApi> = vec![
        ApiConfig::openapi(),
        AuthApi::openapi(),
        UserApi::openapi(),
    
    ];

    let mut all_api = register.into_iter();
    let mut mergetd_api = all_api.next().unwrap();

    for api in all_api {
        mergetd_api.merge(api);
    }

    mergetd_api
}
