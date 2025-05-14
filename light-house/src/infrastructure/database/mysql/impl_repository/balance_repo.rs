use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use crate::domain::entities::asset;
use crate::domain::req_repository::balance_repository::BalanceRepositoryUtill;
use crate::{domain::{entities::current_sheet, req_repository::balance_repository::BalanceRepositoryBase}, soc::soc_repository::RepositoryError};



pub struct BalanceRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl BalanceRepositoryBase for BalanceRepositoryImpl {
    async fn get_current_sheet_by_asset_id(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> Result<Option<current_sheet::Model>, RepositoryError> {
        // Query the database to find the current sheet by asset ID and ensure it belongs to the user
        let current_sheet = current_sheet::Entity::find()
            .filter(current_sheet::Column::AssetId.eq(asset_id.as_bytes().to_vec()))
            .filter(current_sheet::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(current_sheet)
    }

    async fn get_all_current_sheets_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<current_sheet::Model>, RepositoryError> {
        // Query the database to retrieve all current sheets for the given user
        let current_sheets = current_sheet::Entity::find()
            .filter(current_sheet::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(current_sheets)
    }

    async fn create_current_sheet(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
        initial_balance: f64,
    ) -> Result<current_sheet::Model, RepositoryError> {
        // Convert f64 to Decimal
        let initial_balance_decimal = Decimal::from_f64(initial_balance)
            .ok_or_else(|| RepositoryError::OperationFailed("Invalid balance value".to_string()))?;
    
        // Create the ActiveModel for the current sheet
        let new_current_sheet = current_sheet::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()),
            asset_id: Set(asset_id.as_bytes().to_vec()),
            balance: Set(initial_balance_decimal), // Use Decimal here
            user_id: Set(user_id.as_bytes().to_vec()),
            last_transaction_id: Set(None), // No transaction yet
            updated_at: Set(Some(chrono::Utc::now())),
        };
    
        // Insert the current sheet into the database
        let inserted_current_sheet = new_current_sheet
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
    
        Ok(inserted_current_sheet)
    }

    async fn update_current_sheet(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
        balance: Option<f64>,
    ) -> Result<current_sheet::Model, RepositoryError> {
        // Ensure the current sheet exists and belongs to the user
        let current_sheet = self
            .get_current_sheet_by_asset_id(user_id, asset_id)
            .await?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!(
                    "Current sheet for asset ID {} not found for user {}",
                    asset_id, user_id
                ))
            })?;
    
        // Convert the existing current sheet into an ActiveModel for updating
        let mut active_model: current_sheet::ActiveModel = current_sheet.into();
    
        // Update the balance if provided
        if let Some(new_balance) = balance {
            let new_balance_decimal = Decimal::from_f64(new_balance)
                .ok_or_else(|| RepositoryError::OperationFailed("Invalid balance value".to_string()))?;
            active_model.balance = Set(new_balance_decimal); // Use Decimal here
        }

    
        // Save the updated current sheet to the database
        let updated_current_sheet = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
    
        Ok(updated_current_sheet)
    }

    async fn delete_current_sheet_by_asset_id(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> Result<(), RepositoryError> {
        // Ensure the current sheet exists and belongs to the user
        let current_sheet_exists = current_sheet::Entity::find()
            .filter(current_sheet::Column::AssetId.eq(asset_id.as_bytes().to_vec()))
            .filter(current_sheet::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if current_sheet_exists.is_none() {
            return Err(RepositoryError::NotFound(format!(
                "Current sheet for asset ID {} not found for user {}",
                asset_id, user_id
            )));
        }

        // Delete the current sheet
        current_sheet::Entity::delete_many()
            .filter(current_sheet::Column::AssetId.eq(asset_id.as_bytes().to_vec()))
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(())
    }
}


#[async_trait::async_trait]
impl BalanceRepositoryUtill for BalanceRepositoryImpl {

    async fn get_all_current_sheets_by_asset_type_id(
        &self, 
        user_id: Uuid, 
        asset_type_id: Uuid
    ) 
        -> Result<Vec<current_sheet::Model>, RepositoryError>
    {
       // Step 1: Query all AssetIds from the Asset table where AssetTypeId matches
       let asset_ids = asset::Entity::find()
       .filter(asset::Column::AssetTypeId.eq(asset_type_id.as_bytes().to_vec()))
       .select_only()
       .column(asset::Column::Id)
       .into_values::<Vec<u8>, asset::Column>()
       .all(self.db_pool.as_ref())
       .await
       .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Convert AssetIds to a vector of UUIDs
        let asset_ids: Vec<Uuid> = asset_ids
            .into_iter()
            .filter_map(|id| Uuid::from_slice(&id).ok())
            .collect();

        if asset_ids.is_empty() {
            // If no matching assets are found, return an empty vector
            return Ok(vec![]);
        }

        // Step 2: Query the CurrentSheet table using the AssetIds and UserId
        let current_sheets = current_sheet::Entity::find()
            .filter(current_sheet::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .filter(current_sheet::Column::AssetId.is_in(asset_ids.into_iter().map(|id| id.as_bytes().to_vec())))
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(current_sheets)
    }
}