use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto}};





#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::contact_route::create_contact,
        crate::infrastructure::http::http_handler::contact_route::view_contact_by_id,
        crate::infrastructure::http::http_handler::contact_route::view_all_contact,
        crate::infrastructure::http::http_handler::contact_route::delete_contact_by_id,
        crate::infrastructure::http::http_handler::contact_route::update_contact
    ),
    components(
        schemas(
                ReqCreateContactDto, 
                ReqUpdateContactDto, 
                ResEntryContactDto, 
                ResListContactDto
        )
    )
)]
pub struct ContactApi;