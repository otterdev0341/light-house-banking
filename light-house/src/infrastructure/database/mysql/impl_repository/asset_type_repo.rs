use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{dto::assest_type_dto::{ReqCreateAssetTypeDto, ReqUpdateAssestTypeDto}, entities::{asset, asset_type}, req_repository::asset_type_repository::{AssetTypeRepositoryBase, AssetTypeRepositoryUtility}}, soc::soc_repository::RepositoryError};






pub struct AssetTypeRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}

impl AssetTypeRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait]
impl AssetTypeRepositoryBase for AssetTypeRepositoryImpl{
    async fn create(&self, user_id: Uuid, dto: ReqCreateAssetTypeDto
    ) -> Result<asset_type::Model, RepositoryError>
    {
        let new_asset_type = asset_type::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the asset type
            name: Set(dto.name),
            user_id: Set(user_id.as_bytes().to_vec()), // Set the user ID
            ..Default::default()
        };
         // Insert the asset type into the database
         let inserted_asset_type = new_asset_type
         .insert(self.db_pool.as_ref())
         .await
         .map_err(|err| {
             if let sea_orm::DbErr::Exec(exec_err) = &err {
                 if exec_err.to_string().contains("UNIQUE") {
                     return RepositoryError::UniqueConstraintViolation(
                         "Asset type name already exists".to_string(),
                     );
                 }
             }
             RepositoryError::DatabaseError(err.to_string())
         })?;

     Ok(inserted_asset_type)
    }
    async fn find_by_id(
        &self, 
        user_id: Uuid, 
        asset_type_id: Uuid
    ) 
        -> Result<Option<asset_type::Model>, RepositoryError>
    {
        // Query the database to find the asset type by ID and ensure it belongs to the user
        let asset_type = asset_type::Entity::find()
            .filter(asset_type::Column::Id.eq(asset_type_id.as_bytes().to_vec())) // Filter by asset type ID
            .filter(asset_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()).to_string());

        // Return the asset type if found, or None if not found
        asset_type.map_err(|err| RepositoryError::DatabaseError(err)) // Convert the error type
    }
    async fn find_all(&self) -> Result<Vec<asset_type::Model>, RepositoryError>
    {
        // Query the database to retrieve all asset types for the given user
        let asset_types = asset_type::Entity::find()
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of asset types
        Ok(asset_types)
    }
    async fn update(
        &self, 
        user_id: Uuid, 
        asset_type_id: Uuid,
        dto: ReqUpdateAssestTypeDto
    ) 
        -> Result<asset_type::Model, RepositoryError>
    {

        // Validate input
        if asset_type_id.is_nil() || user_id.is_nil() {
        return Err(RepositoryError::InvalidInput("Invalid asset_type_id or user_id".to_string()));
    }

         // Find the asset type by ID and ensure it belongs to the user
         let asset_type = asset_type::Entity::find()
            .filter(asset_type::Column::Id.eq(asset_type_id.as_bytes().to_vec())) // Correctly use asset_type_id
            .filter(asset_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Correctly use user_id
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        log::debug!("Updating asset type with ID: {:?} for user ID: {:?}", asset_type_id, user_id);
        // Return an error if the asset type is not found
        let asset_type = match asset_type {
            Some(asset_type) => asset_type,
            None => {
                return Err(RepositoryError::NotFound(format!(
                    "Asset type with ID {} not found for user {}",
                    asset_type_id, user_id
                )))
            }
        };

        // Convert the found asset type into an ActiveModel for updating
        let mut active_model: asset_type::ActiveModel = asset_type.into();

        // Update fields if they are provided in the DTO
        if let Some(name) = dto.name {
            if !name.is_empty() {
                active_model.name = Set(name);
            }
        }

        // Save the updated asset type to the database
        let updated_asset_type = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Asset type name already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(updated_asset_type)
    }
    
    async fn delete(&self,user_id: Uuid, asset_type_id : Uuid) -> Result<(), RepositoryError>
    {
        // Attempt to delete the asset type by ID and ensure it belongs to the user
        let result = asset_type::Entity::delete_many()
            .filter(asset_type::Column::Id.eq(asset_type_id.as_bytes().to_vec())) // Filter by asset type ID
            .filter(asset_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Check if any rows were affected (i.e., if the asset type was deleted)
        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound(format!(
                "Asset type with ID {} not found for user {}",
                asset_type_id, user_id
            )));
        }

        Ok(())
    }
}


#[async_trait::async_trait]
impl AssetTypeRepositoryUtility for AssetTypeRepositoryImpl{
    async fn is_in_use(&self, user_id: Uuid, asset_type_id: Uuid) -> Result<bool, RepositoryError>
    {
        // Query the database to check if the asset type is in use
        let count = asset::Entity::find() // Replace with the correct module/entity name
            .filter(asset::Column::AssetTypeId.eq(asset_type_id.as_bytes().to_vec())) // Filter by asset type ID
            .filter(asset::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .count(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // If the count is greater than 0, the asset type is in use
        Ok(count > 0)
    }
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset_type::Model>, RepositoryError>
    {
        // Query the database to retrieve all asset types for the given user
        let asset_types = asset_type::Entity::find()
            .filter(asset_type::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()));

        // Return the list of asset types
        asset_types.map_err(|err| RepositoryError::OperationFailed(err.to_string())) // Convert the error type
    }
}