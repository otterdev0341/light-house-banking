use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::transaction_dto::{ReqCreatePaymentDto, ReqUpdatePaymentDto, ResEntryPaymentDto}};





#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::transaction::payment_route::create_payment,
        crate::infrastructure::http::http_handler::transaction::payment_route::get_payment,
        crate::infrastructure::http::http_handler::transaction::payment_route::update_payment,
        crate::infrastructure::http::http_handler::transaction::payment_route::delete_payment,
        crate::infrastructure::http::http_handler::transaction::payment_route::get_all_payments
    ),
    components(
        schemas(
            ReqCreatePaymentDto,
            ReqUpdatePaymentDto,
            ResEntryPaymentDto
        )
    )
)]
pub struct PaymentApi;