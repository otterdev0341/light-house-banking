use light_house::{configuration::mysql_config::DatabaseConfig, domain::migration::Migrator, infrastructure::{database::mysql::mysql_connection, http::faring::cors::CORS}};
use rocket::{get, routes};
use sea_orm_migration::MigratorTrait;



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
    Ok(())
}
