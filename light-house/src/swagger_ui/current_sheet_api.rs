use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::current_sheet_dto::{ResCurrentSheetDto, ResListCurrentSheetDto}};





#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::current_sheet_route::fetch_current_sheet_by_current_sheet_by_id,
        crate::infrastructure::http::http_handler::current_sheet_route::fetch_all_current_sheets_by_user_id,
        crate::infrastructure::http::http_handler::current_sheet_route::fetch_all_current_sheets_by_asset_id
    ),
    components(
        schemas(
            ResCurrentSheetDto,
            ResListCurrentSheetDto
        )
    )
)]
pub struct CurrentSheetApi;