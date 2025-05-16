use std::sync::Arc;

use light_house::{configuration::mysql_config::DatabaseConfig, domain::migration::Migrator, infrastructure::{database::mysql::mysql_connection, http::faring::cors::CORS}, initiation::init_usecase_setup::init_usecase_setup};
use rocket::{get, routes};
use sea_orm_migration::MigratorTrait;
use light_house::initiation::init_handler_setup::init_handler_setup;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error>  {
    
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();
    // Initialize the database connection pool
    let config = DatabaseConfig::default();
    let db = mysql_connection::connect(&config).await.unwrap();
    tracing::info!("Database connection established");
    // initialize database migrations
    tracing::info!("Running database migrations");
    Migrator::up(&db, None).await.unwrap();
    // Migrator::fresh(&db).await.unwrap();
    tracing::info!("Database migrations completed");
    let db_arc = Arc::new(db);

    match rocket::build()
        .attach(CORS)
        .attach(init_usecase_setup(Arc::clone(&db_arc)))
        .mount("/", routes![index])
        .attach(init_handler_setup())
        .launch()
        .await {
        Ok(_) => {
            tracing::info!("Rocket server started successfully");
        },
        Err(e) => {
            tracing::error!("Rocket server failed to start: {}", e);
        }
    }
    Ok(())
}
