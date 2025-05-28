use rocket::{http::Status, outcome::Outcome, request::{self, FromRequest}, Request, State};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;
use crate::domain::entities::user;

pub struct McpAuthenticateUser {
    pub user_id: Uuid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for McpAuthenticateUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Step 1: Extract the database connection from Rocket's State
        let db_pool = match req.rocket().state::<State<DatabaseConnection>>() {
            Some(state) => state.inner(),
            None => {
                return Outcome::Error((
                    Status::InternalServerError,
                    "Database connection not available".to_string(),
                ));
            }
        };

        // Step 2: Extract the MCP token from the header
        if let Some(auth_header) = req.headers().get_one("Mcp Authorization") {
            if let Some(token) = auth_header.strip_prefix("Mcp Token ") {
                // Step 3: Query the user table to find the user with the given MCP token
                match user::Entity::find()
                    .filter(user::Column::McpToken.eq(token.to_string()))
                    .one(db_pool)
                    .await
                {
                    Ok(Some(user)) => {
                        // Step 4: Return the user ID as McpAuthenticateUser
                        match Uuid::from_slice(&user.id) {
                            Ok(user_id) => Outcome::Success(McpAuthenticateUser { user_id }),
                            Err(_) => Outcome::Error((
                                Status::InternalServerError,
                                "Failed to parse user ID".to_string(),
                            )),
                        }
                    }
                    Ok(None) => Outcome::Error((
                        Status::Unauthorized,
                        "Invalid MCP token".to_string(),
                    )),
                    Err(err) => {
                        eprintln!("Database error: {}", err);
                        Outcome::Error((
                            Status::InternalServerError,
                            "Database query failed".to_string(),
                        ))
                    }
                }
            } else {
                Outcome::Error((
                    Status::Unauthorized,
                    "Authorization header malformed".to_string(),
                ))
            }
        } else {
            Outcome::Error((
                Status::Unauthorized,
                "Authorization header missing".to_string(),
            ))
        }
    }
}