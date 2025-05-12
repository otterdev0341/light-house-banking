use light_house::infrastructure::http::faring::cors::CORS;
use rocket::{get, routes};
#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main()  {
    
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();
    // Initialize the database connection pool

    // initialize database migrations
    

    match rocket::build()
        .attach(CORS)
        .mount("/", routes![index])
        .launch()
        .await {
        Ok(_) => {
            tracing::info!("Rocket server started successfully");
        },
        Err(e) => {
            tracing::error!("Rocket server failed to start: {}", e);
        }
    }
    
}
