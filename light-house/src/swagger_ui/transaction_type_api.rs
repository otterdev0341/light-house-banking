use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::transaction_type_dto::ResListTransactionTypeDto};





#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::transaction::transaction_type::fetch_all_transaction_type,
        
    ),
    components(
        schemas(
            ResListTransactionTypeDto
        )
    )
)]
pub struct TransactionTypeApi;