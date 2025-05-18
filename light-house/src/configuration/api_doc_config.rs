use utoipa::OpenApi;




#[derive(OpenApi)]
#[openapi(
    info(
        title = "Light House API",
        version = "0.1.0",
        description = "Collection of APIs for Light House",
    ),
    servers(
        (url = "http://127.0.0.1:8000/v1", description = "Local Development Server"),
        
    ),
)]
pub struct ApiConfig;