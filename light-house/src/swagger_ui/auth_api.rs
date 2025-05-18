use utoipa::OpenApi;

use crate::domain::dto::auth_dto::{ReqSignInDto, ReqSignUpDto, ResMeDto};




#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(),
    paths(
        crate::infrastructure::http::http_handler::user_route::sign_in,
        crate::infrastructure::http::http_handler::user_route::sign_up,
        
        
    ),
    components(
        schemas(
            ReqSignUpDto,
            ResMeDto,
            ReqSignInDto
        )
    )
)]
pub struct AuthApi;