pub struct JwtSecret {
    pub jwt_secret: String
}

impl Default for JwtSecret {
    fn default() -> Self {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable is not set");
        Self { jwt_secret }
    }
}