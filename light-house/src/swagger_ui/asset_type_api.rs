use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::assest_type_dto::{ReqCreateAssetTypeDto, ReqUpdateAssestTypeDto, ResEntryAssetTypeDto, ResListAssestTypeDto}};



#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::asset_type_route::create_asset_type,
        crate::infrastructure::http::http_handler::asset_type_route::view_asset_type_by_id,
        crate::infrastructure::http::http_handler::asset_type_route::view_all_asset_types,
        crate::infrastructure::http::http_handler::asset_type_route::delete_asset_type_by_id,
        crate::infrastructure::http::http_handler::asset_type_route::update_asset_type
    ),components(
        schemas(
            ReqCreateAssetTypeDto,
            ReqUpdateAssestTypeDto,
            ResListAssestTypeDto,
            ResEntryAssetTypeDto
        )
    )
)]
pub struct AssetTypeApi;