use rocket::fairing::AdHoc;

use crate::infrastructure::http::http_handler::{asset_route::asset_routes, asset_type_route::asset_type_routes, contact_route::contact_routes, contact_type_route::contact_type_routes, user_route::user_routes};




pub fn init_handler_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize handlers",  |rocket | async {
        rocket
            .mount("/v1", user_routes())
            .mount("/v1/asset-type",asset_type_routes())
            .mount("/v1/asset", asset_routes())
            .mount("/v1/contact-type", contact_type_routes())
            .mount("/v1/contact", contact_routes())
    })
}