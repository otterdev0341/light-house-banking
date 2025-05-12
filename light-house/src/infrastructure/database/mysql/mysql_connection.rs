

use sea_orm::*;

use crate::configuration::mysql_config::DatabaseConfig;





pub async fn connect(config: &DatabaseConfig)-> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(
        format!("mysql://{}:{}@{}:{}/{}",
        config.db_username,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_database_name
        
    ));

    opts.sqlx_logging(false);
    Database::connect(opts).await
}