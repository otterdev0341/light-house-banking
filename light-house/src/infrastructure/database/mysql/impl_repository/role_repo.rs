use std::sync::Arc;

use sea_orm::{ ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, RelationTrait};
use uuid::Uuid;
use crate::{domain::{entities::{user, user_role}, req_repository::user_role_repository::RoleManagementRepository}, soc::soc_repository::RepositoryError};




pub struct RoleManagementRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>,
}

impl RoleManagementRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait]
impl RoleManagementRepository for RoleManagementRepositoryImpl {


    async fn has_role(
        &self, 
        user_id: Uuid, 
        role: &str
    ) 
        -> Result<bool, RepositoryError>
    {
        // Query the database to check if the user has the specified role
        let user_with_role = user::Entity::find()
            .filter(user::Column::Id.eq(user_id.as_bytes().to_vec())) // Filter by user_id
            .join(sea_orm::JoinType::InnerJoin, user_role::Relation::User.def()) // Join with the user_role table
            .filter(user_role::Column::Name.eq(role)) // Filter by role name
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return true if the user has the role, otherwise false
        Ok(user_with_role.is_some())
    }


    async fn assign_role(
        &self, 
        admin_id: Uuid, 
        target_user_id: Uuid, 
        role: &str
    ) 
        -> Result<(), RepositoryError>
    {
        // Step 1: Ensure the admin has the "admin" role
        let is_admin = self.has_role(admin_id, "admin").await?;
        if !is_admin {
            return Err(RepositoryError::PermissionDenied(
                "Only admins can assign roles".to_string(),
            ));
        }

        // Step 2: Find the role in the user_role table
        let role_entity = user_role::Entity::find()
            .filter(user_role::Column::Name.eq(role))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        let role_entity = match role_entity {
            Some(role) => role,
            None => {
                return Err(RepositoryError::NotFound(format!(
                    "Role '{}' not found",
                    role
                )));
            }
        };

        // Step 3: Update the target user's role
        let update_result = user::Entity::update_many()
            .col_expr(user::Column::UserRoleId, sea_orm::sea_query::Expr::value(role_entity.id))
            .filter(user::Column::Id.eq(target_user_id.as_bytes().to_vec()))
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Step 4: Check if the update affected any rows
        if update_result.rows_affected == 0 {
            return Err(RepositoryError::NotFound(format!(
                "User with ID '{}' not found",
                target_user_id
            )));
        }

        Ok(())
    }


    async fn revoke_role(
        &self, 
        admin_id: Uuid, 
        target_user_id: Uuid, 
        role: &str
    ) 
        -> Result<(), RepositoryError>
    {
        // Step 1: Ensure the admin has the "admin" role
        let is_admin = self.has_role(admin_id, "admin").await?;
        if !is_admin {
            return Err(RepositoryError::PermissionDenied(
                "Only admins can revoke roles".to_string(),
            ));
        }

        // Step 2: Check if the target user currently has the specified role
        let has_role = self.has_role(target_user_id, role).await?;
        if !has_role {
            return Err(RepositoryError::NotFound(format!(
                "User with ID '{}' does not have the role '{}'",
                target_user_id, role
            )));
        }

        // Step 3: Find the default role (e.g., "user") in the user_role table
        let default_role = user_role::Entity::find()
            .filter(user_role::Column::Name.eq("user"))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        let default_role = match default_role {
            Some(role) => role,
            None => {
                return Err(RepositoryError::NotFound(
                    "Default role 'user' not found".to_string(),
                ));
            }
        };

        // Step 4: Update the target user's role to the default role
        let update_result = user::Entity::update_many()
            .col_expr(user::Column::UserRoleId, sea_orm::sea_query::Expr::value(default_role.id))
            .filter(user::Column::Id.eq(target_user_id.as_bytes().to_vec()))
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Step 5: Check if the update affected any rows
        if update_result.rows_affected == 0 {
            return Err(RepositoryError::NotFound(format!(
                "User with ID '{}' not found",
                target_user_id
            )));
        }

        Ok(())
    }

    async fn get_role_by_id(&self, role_id: Uuid) -> Result<Option<user_role::Model>, RepositoryError>
    {
      // Query the database to find the role by its ID
      let role = user_role::Entity::find_by_id(role_id.as_bytes().to_vec())
        .one(self.db_pool.as_ref())
        .await
        .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

      Ok(role)
    }
}