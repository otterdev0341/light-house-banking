use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{http::Status, request::{self, FromRequest, Outcome}, Request};

use crate::{configuration::jwt_config::JwtSecret, domain::dto::auth_dto::Claims};

pub struct AuthenticatedUser {
    pub id: i32,
    
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(auth_header) = req.headers().get_one("Authorization") {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                let config = match req.rocket().state::<JwtSecret>() {
                    Some(config) => config,
                    None => return Outcome::Error((Status::InternalServerError, "JWT secret not configured".to_string())),
                };

                let data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                    &Validation::new(jsonwebtoken::Algorithm::HS512),
                );

                let claims = match data {
                    Ok(p) => p.claims,
                    Err(_) => {
                        return Outcome::Error((Status::Unauthorized, "Invalid token".to_string()))
                    }
                };            

                return Outcome::Success(AuthenticatedUser { id: claims.sub });
            }
        }
        Outcome::Error((Status::Unauthorized, "Authorization header missing or malformed".to_string()))
    }
}