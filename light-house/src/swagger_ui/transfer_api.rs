use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::transaction_dto::{ReqCreateTransferDto, ReqUpdateTransferDto, ResEntryTransferDto, ResListTransferDto}};









#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::transaction::transfer_route::create_tranfer,
        crate::infrastructure::http::http_handler::transaction::transfer_route::get_transfer,
        crate::infrastructure::http::http_handler::transaction::transfer_route::update_transfer,
        crate::infrastructure::http::http_handler::transaction::transfer_route::delete_transfer,
        crate::infrastructure::http::http_handler::transaction::transfer_route::get_all_transfer
    ),
    components(
        schemas(
            ReqCreateTransferDto, 
            ReqUpdateTransferDto, 
            ResEntryTransferDto, 
            ResListTransferDto
        )
    )
)]
pub struct TransferApi;