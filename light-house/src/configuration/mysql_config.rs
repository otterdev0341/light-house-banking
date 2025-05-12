pub struct DatabaseConfig {
    pub db_username: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_database_name: String,
}
impl Default for DatabaseConfig {
    fn default() -> Self {

        let username = std::env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string());
        let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "root".to_string());
        let host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("DB_PORT")
            .unwrap_or_else(|_| "3306".to_string())
            .parse::<u16>()
            .unwrap_or(3306);
        let database_name = std::env::var("DB_NAME")
            .unwrap_or_else(|_| "light_house".to_string());
        
        Self {
            db_username: username,
            db_password: password,
            db_host: host,
            db_port: port,
            db_database_name: database_name,
        }
    }

    
}

impl DatabaseConfig {
    pub fn get_database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.db_username, self.db_password, self.db_host, self.db_port, self.db_database_name
        )
    }
}