use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::transaction_dto::{ReqCreateIncomeDto, ReqUpdateIncomeDto, ResEntryIncomeDto, ResListIncomeDto}};



#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::transaction::income_route::create_income,
        crate::infrastructure::http::http_handler::transaction::income_route::get_income,
        crate::infrastructure::http::http_handler::transaction::income_route::update_income,
        crate::infrastructure::http::http_handler::transaction::income_route::delete_income,
        crate::infrastructure::http::http_handler::transaction::income_route::get_all_income
    ),
    components(
        schemas(
            ReqCreateIncomeDto, 
            ReqUpdateIncomeDto, 
            ResEntryIncomeDto, 
            ResListIncomeDto
        )
    )
)]
pub struct IncomeApi;