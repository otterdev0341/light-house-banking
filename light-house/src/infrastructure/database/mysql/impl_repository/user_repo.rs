use std::sync::Arc;

use bcrypt::{hash, DEFAULT_COST};
use rand::{distr::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use uuid::Uuid;

use crate::{ domain::{dto::auth_dto::{ReqSignUpDto, ReqUpdateUserDto}, entities::{gender, user, user_role}, req_repository::user_repository::{McpRepositoryBase, UserRepositoryBase, UserRepositoryUtility}}, soc::soc_repository::RepositoryError};








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


    async fn create(&self, dto: ReqSignUpDto) -> Result<user::Model, RepositoryError> {
        log::debug!("Starting create function for username: {}", dto.username);

        // Hash the password
        let hashed_password = hash(&dto.password, DEFAULT_COST)
            .map_err(|_| RepositoryError::InvalidInput("Failed to hash password".to_string()))?;
        log::debug!("Password hashed successfully");

        // Query the gender table
        let gender = gender::Entity::find()
            .filter(gender::Column::Name.eq(dto.gender.clone()))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        log::debug!("Gender query completed");

        let gender_id = match gender {
            Some(gender) => gender.id,
            None => {
                log::error!("Gender '{}' not found", dto.gender);
                return Err(RepositoryError::InvalidInput(format!(
                    "Gender '{}' not found",
                    dto.gender
                )));
            }
        };

        // Query the user_role table
        let role = user_role::Entity::find()
            .filter(user_role::Column::Name.eq("user"))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        log::debug!("Role query completed");

        let role_id = match role {
            Some(role) => role.id,
            None => {
                log::error!("Role 'user' not found");
                return Err(RepositoryError::InvalidInput(
                    "Role 'user' not found".to_string(),
                ));
            }
        };

        let mcp_token = Uuid::now_v7().to_string();
        log::debug!("Generated UUID v7 mcp_token: {}", mcp_token);


        // Create the ActiveModel for the user
        let new_user = user::ActiveModel {
            id: Set(Uuid::new_v4().into()),
            username: Set(dto.username),
            email: Set(dto.email),
            password: Set(hashed_password),
            first_name: Set(dto.first_name),
            last_name: Set(dto.last_name),
            gender_id: Set(gender_id.into()),
            user_role_id: Set(role_id.into()),
            mcp_token: Set(mcp_token),
            ..Default::default()
        };
        log::debug!("New user ActiveModel created");

        // Insert the user into the database
        let inserted_user = new_user
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                log::error!("Error inserting user: {}", err);
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Username or email already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;
        log::debug!("User inserted successfully");

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

        // Helper function to check if a field should be updated
        fn should_update(field: &Option<String>) -> Option<String> {
            field.as_ref().filter(|value| !value.is_empty()).cloned()
        }

        // Update fields if they are provided in the DTO and are not empty strings
        if let Some(username) = should_update(&dto.username) {
            active_model.username = Set(username);
        }
        if let Some(email) = should_update(&dto.email) {
            active_model.email = Set(email);
        }
        if let Some(first_name) = should_update(&dto.first_name) {
            active_model.first_name = Set(first_name);
        }
        if let Some(last_name) = should_update(&dto.last_name) {
            active_model.last_name = Set(last_name);
        }
        if let Some(password) = should_update(&dto.password) {
            // Hash the password before updating
            let hashed_password = hash(&password, DEFAULT_COST)
                .map_err(|_| RepositoryError::InvalidInput("Failed to hash password".to_string()))?;
            active_model.password = Set(hashed_password);
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
impl McpRepositoryBase for UserRepositoryImpl{
    
    async fn get_user_id_from_mcp_token(&self, mcp_token: &str) -> Result<user::Model, RepositoryError>
    {   
        // Query the database to find the user by MCP token
        let user = user::Entity::find()
            .filter(user::Column::McpToken.eq(mcp_token)) // Filter by the `mcp_token` column
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the user if found, or an error if not found
        match user {
            Some(user) => Ok(user),
            None => Err(RepositoryError::NotFound(format!("User with MCP token '{}' not found", mcp_token))),
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


