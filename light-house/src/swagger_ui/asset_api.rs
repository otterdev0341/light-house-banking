use utoipa::OpenApi;

use crate::{configuration::api_security_addon::SecurityAddon, domain::dto::asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto, ResEntryAssetDto, ResListAssetDto}};






#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(&SecurityAddon),
    paths(
        crate::infrastructure::http::http_handler::asset_route::create_asset_type,
        crate::infrastructure::http::http_handler::asset_route::view_asset_by_id,
        crate::infrastructure::http::http_handler::asset_route::view_all_asset,
        crate::infrastructure::http::http_handler::asset_route::delete_asset_by_id,
        crate::infrastructure::http::http_handler::asset_route::update_asset
    ),
    components(
        schemas(
            ReqCreateAssetDto, 
            ReqUpdateAssetDto, 
            ResEntryAssetDto, 
            ResListAssetDto
        )
    )
)]
pub struct AssetApi;