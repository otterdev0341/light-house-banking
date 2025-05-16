use rocket::fairing::AdHoc;

use crate::infrastructure::http::http_handler::user_route::user_routes;




pub fn init_handler_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize handlers",  |rocket | async {
        rocket
            .mount("/user/v1", user_routes())
    })
}