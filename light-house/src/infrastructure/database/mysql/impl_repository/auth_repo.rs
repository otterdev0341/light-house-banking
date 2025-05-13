use std::{sync::Arc, time::SystemTime};

use jsonwebtoken::crypto::verify;
use rocket::http::Status;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use uuid::Uuid;

use crate::{configuration::jwt_config::JwtSecret, domain::{dto::auth_dto::{Claims, ReqSignInDto, ReqSignUpDto, ReqUpdateUserDto, ResMeDto, ResSignInDto}, entities::{gender, user, user_role}, req_repository::{auth_repository::AuthRepository}}, infrastructure::http::response::otter_response::ErrorResponse};








pub struct AuthRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}

impl AuthRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}

// impl trait
#[async_trait::async_trait]
impl AuthRepository for AuthRepositoryImpl {
    
    async fn sign_in(&self, sign_in_dto: ReqSignInDto
    ) -> Result<rocket::serde::json::Json<ResSignInDto> , String>
    {
        // create connection
        let conn = Arc::clone(&self.db_pool);
        let jwt_config = JwtSecret::default();
        // find user by email
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(sign_in_dto.email))
            .one(conn.as_ref())
            .await
            .map_err(|_| "Error while finding user".to_string())?;
        // check is user exist
        let user = match user {
            Some(user) => user,
            None => return Err("User not found".to_string()),
        };
        // check is password correct
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(jwt_config.jwt_secret.as_bytes());
        let is_password_correct = verify(
            &user.password,
            sign_in_dto.password.as_bytes(),
            &decoding_key,
            jsonwebtoken::Algorithm::HS256,
        ).map_err(|_| "Error while verifying password".to_string())?;

        // return error if password is incorrect
        if !is_password_correct {
            return Err("Password is incorrect".to_string());
        }


        // generate claim 
        let claim = Claims{
            sub: Uuid::from_slice(&user.id).map_err(|_| "Invalid UUID format".to_string())?,
            role: "user".to_string(),
            exp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| format!(
                    "Time error: {:?}",
                    ErrorResponse(Status::InternalServerError, format!("{:?}", e))
                ))?
                // Ensure the error type matches the function's declared error type (String)
                .as_secs() + 4 * 60 * 60
                
        };

        // generate token
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claim,
            &jsonwebtoken::EncodingKey::from_secret(jwt_config.jwt_secret.as_bytes()),
        ).map_err(|_| "Error while generating token".to_string())?;
        // return token
        let res = ResSignInDto {
            token
        };
        Ok(rocket::serde::json::Json(res))

    }

}



impl AuthRepositoryImpl {
    async fn me(&self, user_id: Uuid) -> Result<ResMeDto, String> {
        // Create connection
        let conn = Arc::clone(&self.db_pool);

        // Find user by ID
        let user = user::Entity::find()
            .filter(user::Column::Id.eq(user_id))
            .one(conn.as_ref())
            .await
            .map_err(|_| "Error while finding user".to_string())?;

        // Check if user exists
        let user = match user {
            Some(user) => user,
            None => return Err("User not found".to_string()),
        };

        // Fetch gender from the Gender table
        let gender = gender::Entity::find()
            .filter(gender::Column::Id.eq(user.gender_id))
            .one(conn.as_ref())
            .await
            .map_err(|_| "Error while finding gender".to_string())?
            .map(|g| g.name)
            .unwrap_or_else(|| "Unknown".to_string());

        // Fetch user role from the UserRole table
        let user_role = user_role::Entity::find()
            .filter(user_role::Column::Id.eq(user.user_role_id))
            .one(conn.as_ref())
            .await
            .map_err(|_| "Error while finding user role".to_string())?
            .map(|r| r.name)
            .unwrap_or_else(|| "Unknown".to_string());

        // Construct the response
        let res = ResMeDto {
            id: Uuid::from_slice(&user.id)
                .map_err(|_| "Invalid UUID format".to_string())?
                .to_string(),
            gender,
            user_role,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        };

        Ok(res)
    }
}
