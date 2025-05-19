use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::expense_type_dto::{ReqCreateExpenseTypeDto, ReqUpdateExpenseTypeDto, ResEntryExpenseTypeDto, ResListExpenseTypeDto}};



#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::expense_type_route::create_expense_type,
        crate::infrastructure::http::http_handler::expense_type_route::view_expense_type_by_id,
        crate::infrastructure::http::http_handler::expense_type_route::view_all_expense_types,
        crate::infrastructure::http::http_handler::expense_type_route::delete_expense_type_by_id,
        crate::infrastructure::http::http_handler::expense_type_route::update_expense_type
    ),
    components(
        schemas(
            ReqCreateExpenseTypeDto, 
            ReqUpdateExpenseTypeDto, 
            ResEntryExpenseTypeDto, 
            ResListExpenseTypeDto
        )
    )
)]
pub struct ExpenseTypeApi;