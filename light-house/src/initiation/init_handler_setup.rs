use rocket::fairing::AdHoc;

use crate::infrastructure::http::http_handler::{asset_route::asset_routes, asset_type_route::asset_type_routes, contact_route::contact_routes, contact_type_route::contact_type_routes, current_sheet_route::current_sheet_routes, expense_route::expense_routes, expense_type_route::expense_type_routes, transaction::{income_route::income_routes, payment_route::payment_routes, transaction_type::transaction_type_routes, transfer_route::transfer_routes}, user_route::user_routes};




pub fn init_handler_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize handlers",  |rocket | async {
        rocket
            .mount("/v1", user_routes())
            .mount("/v1/asset-type",asset_type_routes())
            .mount("/v1/asset", asset_routes())
            .mount("/v1/contact-type", contact_type_routes())
            .mount("/v1/contact", contact_routes())
            .mount("/v1/expense-type", expense_type_routes())
            .mount("/v1/expense", expense_routes())
            .mount("/v1/income", income_routes())
            .mount("/v1/transaction-type", transaction_type_routes())
            .mount("/v1/current-sheet", current_sheet_routes())
            .mount("/v1/payment", payment_routes())
            .mount("/v1/transfer",transfer_routes())
    })
}