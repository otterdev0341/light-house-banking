use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{dto::asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto}, entities::asset, req_repository::{asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}, balance_repository::BalanceRepositoryBase}}, soc::soc_repository::RepositoryError};

use super::balance_repo::BalanceRepositoryImpl;






pub struct AssetRepositoryImpl{
    pub db_pool: Arc<DatabaseConnection>
}

impl AssetRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait]
impl AssetRepositoryBase for AssetRepositoryImpl{
    async fn create(
        &self, 
        user_id: Uuid, 
        dto: ReqCreateAssetDto
    ) 
        -> Result<asset::Model, RepositoryError>
    {
         // Create the ActiveModel for the asset
        let new_asset = asset::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the asset
            name: Set(dto.name),
            asset_type_id: Set(dto.asset_type_id.as_bytes().to_vec()), // Set the asset type ID
            user_id: Set(user_id.as_bytes().to_vec()), // Set the user ID
            ..Default::default()
        };

        // Insert the asset into the database
        let inserted_asset = new_asset
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Asset name already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        // Create a corresponding CurrentSheet record with an initial balance of 0
        let balance_repo = BalanceRepositoryImpl {
            db_pool: Arc::clone(&self.db_pool),
        };

        balance_repo
            .create_current_sheet(
                user_id,
                Uuid::from_slice(&inserted_asset.id).unwrap(),
                0.0, // Initial balance is 0
            )
            .await?;

        Ok(inserted_asset)
    }


    async fn find_by_id(
        &self, 
        user_id: Uuid, 
        asset_id: Uuid
    ) 
        -> Result<Option<asset::Model>, RepositoryError>
    {
        // Query the database to find the asset by ID and ensure it belongs to the user
        let asset = asset::Entity::find()
            .filter(asset::Column::Id.eq(asset_id.as_bytes().to_vec())) // Filter by asset ID
            .filter(asset::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the asset if found, or None if not found
        Ok(asset)
    }


    async fn find_all(
        &self
    ) 
        -> Result<Vec<asset::Model>, RepositoryError>
    {
        // Query the database to retrieve all assets
        let assets = asset::Entity::find()
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of assets
        Ok(assets)
    }


    async fn update(
        &self, 
        dto: ReqUpdateAssetDto, 
        user_id: Uuid, 
        asset_id: Uuid
    ) 
    -> Result<asset::Model, RepositoryError>
    {
        // Find the asset by ID and ensure it belongs to the user
        let asset = asset::Entity::find()
            .filter(asset::Column::Id.eq(asset_id.as_bytes().to_vec())) // Filter by asset ID
            .filter(asset::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return an error if the asset is not found
        let asset = match asset {
            Some(asset) => asset,
            None => {
                return Err(RepositoryError::NotFound(format!(
                    "Asset with ID {} not found for user {}",
                    asset_id, user_id
                )))
            }
        };

        // Convert the found asset into an ActiveModel for updating
        let mut active_model: asset::ActiveModel = asset.into();

        // Update fields if they are provided in the DTO
        if let Some(name) = dto.name {
            if !name.is_empty() {
                active_model.name = Set(name);
            }
        }

        if let Some(asset_type_id) = dto.asset_type_id {
            active_model.asset_type_id = Set(
                Uuid::parse_str(&asset_type_id)
                    .map_err(|err| RepositoryError::InvalidInput(err.to_string()))?
                    .as_bytes()
                    .to_vec(),
            );
        }

        // Save the updated asset to the database
        let updated_asset = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("UNIQUE") {
                        return RepositoryError::UniqueConstraintViolation(
                            "Asset name already exists".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        // No need to update the CurrentSheet here since the balance is not affected

        Ok(updated_asset)
    }



    async fn delete(
        &self,
        user_id: Uuid, 
        asset_id : Uuid
    ) 
        -> Result<(), RepositoryError>
    {
         // Attempt to delete the asset by ID and ensure it belongs to the user
        let result = asset::Entity::delete_many()
        .filter(asset::Column::Id.eq(asset_id.as_bytes().to_vec())) // Filter by asset ID
        .filter(asset::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
        .exec(self.db_pool.as_ref())
        .await
        .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Check if any rows were affected (i.e., if the asset was deleted)
        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound(format!(
                "Asset with ID {} not found for user {}",
                asset_id, user_id
            )));
        }

        // Delete the corresponding CurrentSheet record
        let balance_repo = BalanceRepositoryImpl {
            db_pool: Arc::clone(&self.db_pool),
        };

        balance_repo
            .delete_current_sheet_by_asset_id(user_id, asset_id)
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AssetRepositoryUtility for AssetRepositoryImpl{

    async fn find_all_by_user_id(
        &self, 
        user_id: Uuid
    ) 
        -> Result<Vec<asset::Model>, String>
    {
        // Query the database to retrieve all assets for the given user
        let assets = asset::Entity::find()
            .filter(asset::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| err.to_string())?;

        // Return the list of assets
        Ok(assets)
    }
}

