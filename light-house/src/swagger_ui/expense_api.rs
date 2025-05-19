use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::expense_dto::{ReqCreateExpenseDto, ReqUpdateExpenseDto, ResEntryExpenseDto, ResListExpenseDto}};





#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::expense_route::create_expense,
        crate::infrastructure::http::http_handler::expense_route::view_expense_by_id,
        crate::infrastructure::http::http_handler::expense_route::view_all_expenses,
        crate::infrastructure::http::http_handler::expense_route::delete_expense_by_id,
        crate::infrastructure::http::http_handler::expense_route::update_expense_by_id
    ),
    components(
        schemas(
                ReqCreateExpenseDto, 
                ReqUpdateExpenseDto, 
                ResEntryExpenseDto, 
                ResListExpenseDto
        )
    )
)]
pub struct ExpenseApi;