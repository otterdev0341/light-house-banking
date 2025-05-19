use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto}};





#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::contact_type_route::create_contact_type,
        crate::infrastructure::http::http_handler::contact_type_route::view_contact_type_by_id,
        crate::infrastructure::http::http_handler::contact_type_route::view_all_contact_types,
        crate::infrastructure::http::http_handler::contact_type_route::delete_contact_type_by_id,
        crate::infrastructure::http::http_handler::contact_type_route::update_contact_type
    ),
    components(
        schemas(
            ReqCreateContactTypeDto, 
            ReqUpdateContactTypeDto, 
            ResEntryContactTypeDto, 
            ResListContactTypeDto
        )
    )
)]
pub struct ContactTypeApi;