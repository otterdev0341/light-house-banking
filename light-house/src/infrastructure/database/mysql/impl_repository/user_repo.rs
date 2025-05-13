use std::sync::Arc;

use bcrypt::{hash, DEFAULT_COST};
use rand::{distr::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use uuid::Uuid;

use crate::{domain::{dto::auth_dto::{ReqSignUpDto, ReqUpdateUserDto}, entities::user, req_repository::user_repository::{UserRepositoryBase, UserRepositoryMcp, UserRepositoryUtility}}, soc::soc_repository::RepositoryError};








pub struct UserRepositoryImpl{
    pub db_pool: Arc<DatabaseConnection>
}

impl UserRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait]
impl UserRepositoryBase for UserRepositoryImpl {


    async fn create(&self, dto: ReqSignUpDto) -> Result<user::Model, RepositoryError>
    {
        // Hash the password
        let hashed_password = hash(&dto.password, DEFAULT_COST)
            .map_err(|_| RepositoryError::InvalidInput("Failed to hash password".to_string()))?;

        // Create the ActiveModel for the user
        let new_user = user::ActiveModel {
            id: Set(Uuid::new_v4().into()), // Generate a new UUID for the user
            username: Set(dto.username),
            email: Set(dto.email),
            password: Set(hashed_password), // Use the hashed password
            first_name: Set(dto.first_name),
            last_name: Set(dto.last_name),
            ..Default::default()
        };

        // Insert the user into the database
        let inserted_user = new_user
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Username or email already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(inserted_user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<user::Model>, RepositoryError> {
        // Query the database to find the user by ID
        let user = user::Entity::find()
            .filter(user::Column::Id.eq(id)) // Filter by the `id` column
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the user if found, or None if not found
        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<user::Model>, RepositoryError>
    {
        // Query the database to retrieve all users
        let users = user::Entity::find()
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of users
        Ok(users)
    }



    async fn update(&self, dto: ReqUpdateUserDto, user_id: Uuid) -> Result<user::Model, RepositoryError>
    {
        // Find the user by ID
        let user = user::Entity::find()
            .filter(user::Column::Id.eq(user_id))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return an error if the user is not found
        let user = match user {
            Some(user) => user,
            None => return Err(RepositoryError::NotFound(format!("User with ID {} not found", user_id))),
        };

        // Convert the found user into an ActiveModel for updating
        let mut active_model: user::ActiveModel = user.into();

        // Update fields if they are provided in the DTO and are not empty strings
        if let Some(username) = dto.username {
            if !username.is_empty() {
                active_model.username = Set(username);
            }
        }
        if let Some(email) = dto.email {
            if !email.is_empty() {
                active_model.email = Set(email);
            }
        }
        if let Some(first_name) = dto.first_name {
            if !first_name.is_empty() {
                active_model.first_name = Set(first_name);
            }
        }
        if let Some(last_name) = dto.last_name {
            if !last_name.is_empty() {
                active_model.last_name = Set(last_name);
            }
        }
        if let Some(password) = dto.password {
            if !password.is_empty() {
                // Hash the password before updating
                let hashed_password = hash(&password, DEFAULT_COST)
                    .map_err(|_| RepositoryError::InvalidInput("Failed to hash password".to_string()))?;
                active_model.password = Set(hashed_password);
            }
        }

        // Update the `updated_at` timestamp
        active_model.updated_at = Set(Some(chrono::Utc::now()));

        // Save the updated user to the database
        let updated_user = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Username or email already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(updated_user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>
    {
        // Attempt to delete the user by ID
        let result = user::Entity::delete_by_id(id)
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Check if any rows were affected (i.e., if the user was deleted)
        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound(format!("User with ID {} not found", id)));
        }

        Ok(())
    }

}


#[async_trait::async_trait]
impl UserRepositoryUtility for UserRepositoryImpl{
    async fn find_by_username(&self, name: &str) -> Result<Option<user::Model>, RepositoryError>
    {
        // Query the database to find the user by username
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(name)) // Filter by the `username` column
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the user if found, or None if not found
        Ok(user)
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<user::Model>, RepositoryError>
    {
        // Query the database to find the user by username
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(email)) // Filter by the `username` column
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the user if found, or None if not found
        Ok(user)
    }
}


#[async_trait::async_trait]
impl UserRepositoryMcp for UserRepositoryImpl{
    async fn get_mcp_by_user_id(&self, user_id: Uuid) -> Result<Option<String>, RepositoryError>
    {
       // Query the database to find the user by ID and retrieve the mcp_token
       let user = user::Entity::find()
            .filter(user::Column::Id.eq(user_id)) // Filter by the `id` column
            .select_only()
            .column(user::Column::McpToken) // Select only the `mcp_token` column
            .into_tuple::<Option<String>>() // Map the result to an Option<String>
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the mcp_token if found and not empty, or an error if the user does not exist
        match user {
            Some(Some(mcp_token)) if !mcp_token.is_empty() => Ok(Some(mcp_token)), // Return if not empty
            Some(Some(_)) => Ok(None), // Return None if mcp_token is empty
            Some(None) => Ok(None), // Return None if mcp_token is NULL
            None => Err(RepositoryError::NotFound(format!("User with ID {} not found", user_id))),
        }
    }
    async fn regenerate_mcp_token(&self, user_id: Uuid) -> Result<(), RepositoryError>
    {
         // Generate a secure random token
         let raw_token: String = rand::rng()
         .sample_iter(&Alphanumeric)
         .take(32) // Generate a 32-character token
         .map(char::from)
         .collect();

     // Hash the token using bcrypt
     let hashed_token = hash(&raw_token, DEFAULT_COST)
         .map_err(|_| RepositoryError::InvalidInput("Failed to hash MCP token".to_string()))?;

     // Find the user by ID
     let user = user::Entity::find()
         .filter(user::Column::Id.eq(user_id))
         .one(self.db_pool.as_ref())
         .await
         .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

     // Return an error if the user is not found
     let user = match user {
         Some(user) => user,
         None => return Err(RepositoryError::NotFound(format!("User with ID {} not found", user_id))),
     };

     // Convert the found user into an ActiveModel for updating
     let mut active_model: user::ActiveModel = user.into();

     // Update the mcp_token field with the hashed token
     active_model.mcp_token = Set(hashed_token);

     // Save the updated user to the database
     active_model
         .update(self.db_pool.as_ref())
         .await
         .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

     // Optionally, return the raw token to the caller (if needed)
     // For security reasons, you might want to avoid returning the raw token
     Ok(())
    }
}