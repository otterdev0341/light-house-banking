use utoipa::OpenApi;
use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::auth_dto::{ReqUpdateUserDto, ResMcpDto, ResMeDto}};

#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::user_route::update_user,
        crate::infrastructure::http::http_handler::user_route::me,
        crate::infrastructure::http::http_handler::user_route::get_mcp,
    ),
    components(
        schemas(
            ReqUpdateUserDto,
            ResMeDto,
            ResMcpDto
            
        )
    )
)]
pub struct UserApi;