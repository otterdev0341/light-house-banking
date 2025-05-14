use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ModelTrait};
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use crate::{
    domain::{dto::transaction_dto::{ReqCreateIncomeDto, ReqUpdateIncomeDto}, entities::transaction, req_repository::{balance_repository::BalanceRepositoryBase, transaction_repository::RecordIncomeRepositoryUtility}},
    infrastructure::database::mysql::impl_repository::balance_repo::BalanceRepositoryImpl,
    soc::soc_repository::RepositoryError
};





pub struct IncomeRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}


impl IncomeRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait]
impl RecordIncomeRepositoryUtility for IncomeRepositoryImpl {

    async fn create_income_record(
        &self, 
        user_id: Uuid, 
        income_record_dto: ReqCreateIncomeDto
    )
         -> Result<transaction::Model, RepositoryError>
    {
        let balance_repo = BalanceRepositoryImpl { db_pool: Arc::clone(&self.db_pool) };

        // 1. Create the ActiveModel for the income record
        let new_income_record = transaction::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the transaction
            transaction_type_id: Set(income_record_dto.transaction_type_id.as_bytes().to_vec()),
            amount: Set(income_record_dto.amount),
            asset_id: Set(income_record_dto.asset_id.as_bytes().to_vec()),
            contact_id: Set(Some(income_record_dto.contact_id.as_bytes().to_vec())),
            note: Set(income_record_dto.note),
            user_id: Set(user_id.as_bytes().to_vec()),
            // created_at and updated_at will be set by default by the database or ActiveModel defaults
            ..Default::default()
        };

        // 2. Insert the income record into the database
        let inserted_income_record = new_income_record
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("FOREIGN KEY") {
                        return RepositoryError::OperationFailed(
                            "Invalid foreign key reference in the income record".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        // 3. Update the current sheet balance
        let asset_id_uuid = income_record_dto.asset_id;
        let current_sheet = balance_repo
            .get_current_sheet_by_asset_id(user_id, Uuid::parse_str(&asset_id_uuid).map_err(|e| RepositoryError::OperationFailed(format!("Invalid asset UUID: {}", e)))?)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet not found for asset {}", asset_id_uuid)))?;
        
        let new_balance = current_sheet.balance + Decimal::from_f64(inserted_income_record.amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert amount to Decimal".to_string()))?;
        
        // Convert Decimal to Option<f64> for update_current_sheet
        let new_balance_f64 = new_balance.to_f64(); 

        balance_repo
            .update_current_sheet(user_id, Uuid::parse_str(&asset_id_uuid).map_err(|e| RepositoryError::OperationFailed(format!("Invalid asset UUID: {}", e)))?, new_balance_f64)
            .await?;


        Ok(inserted_income_record)
    }

    
    async fn update_income_record(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid, income_record_dto: ReqUpdateIncomeDto
    ) -> Result<transaction::Model, RepositoryError> {
        let balance_repo = BalanceRepositoryImpl { db_pool: Arc::clone(&self.db_pool) };

        // 1. Fetch the original transaction to get old_amount and old_asset_id
        let original_transaction = transaction::Entity::find_by_id(transaction_id.as_bytes().to_vec())
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound(format!("Transaction {} not found for user {}", transaction_id, user_id)))?;

        let old_amount = original_transaction.amount;
        let old_asset_id_uuid = Uuid::from_slice(&original_transaction.asset_id)
            .map_err(|e| RepositoryError::OperationFailed(format!("Invalid old asset UUID: {}", e)))?;

        // 2. Prepare and execute the update for the transaction record itself
        let mut active_model: transaction::ActiveModel = original_transaction.into();
        if let Some(amount) = income_record_dto.amount {
            active_model.amount = Set(amount);
        }
        if let Some(asset_id) = income_record_dto.asset_id {
            active_model.asset_id = Set(asset_id.as_bytes().to_vec());
        }
        // contact_id can be None, so handle it carefully if it's optional in DTO
        active_model.contact_id = match income_record_dto.contact_id {
            Some(id) => Set(Some(id.as_bytes().to_vec())),
            None => if active_model.contact_id.is_set() || active_model.contact_id.is_unchanged() { 
                        // If it was already set and DTO provides None, it means we want to set it to NULL
                        // However, if DTO field is Option<Option<Uuid>>, this logic needs adjustment.
                        // Assuming ReqUpdateIncomeDto.contact_id: Option<Uuid> means "if Some, update, if None, don't change"
                        // If ReqUpdateIncomeDto.contact_id: Option<Option<Uuid>> then Some(None) would mean set to NULL.
                        // For now, let's assume Option<Uuid> means update if Some.
                        active_model.contact_id // Keep as is if DTO field is None
                    } else {
                        Set(None) // Default to None if not previously set and DTO is None
                    }
        };
        if let Some(note) = income_record_dto.note {
            active_model.note = Set(note);
        }
        active_model.updated_at = Set(Some(chrono::Utc::now())); // Update timestamp

        let updated_transaction = active_model.update(self.db_pool.as_ref()).await.map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // 3. Get new amount and new asset_id
        let new_amount = updated_transaction.amount;
        let new_asset_id_uuid = Uuid::from_slice(&updated_transaction.asset_id)
            .map_err(|e| RepositoryError::OperationFailed(format!("Invalid new asset UUID: {}", e)))?;

        // 4. Update balances
        if old_asset_id_uuid == new_asset_id_uuid {
            // Asset ID hasn't changed, calculate net change
            let balance_change = new_amount - old_amount;
            let current_sheet = balance_repo
                .get_current_sheet_by_asset_id(user_id, new_asset_id_uuid)
                .await?
                .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for asset {} not found", new_asset_id_uuid)))?;
            
            let final_balance = current_sheet.balance + Decimal::from_f64(balance_change).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert balance change to Decimal".to_string()))?;
            balance_repo
                .update_current_sheet(user_id, new_asset_id_uuid, final_balance.to_f64())
                .await?;
        } else {
            // Asset ID has changed:
            // 4a. Revert old amount from old asset's balance
            let old_asset_sheet = balance_repo
                .get_current_sheet_by_asset_id(user_id, old_asset_id_uuid)
                .await?
                .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for old asset {} not found", old_asset_id_uuid)))?;
            
            let balance_after_reverting_old = old_asset_sheet.balance - Decimal::from_f64(old_amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert old_amount to Decimal".to_string()))?;
            balance_repo
                .update_current_sheet(user_id, old_asset_id_uuid, balance_after_reverting_old.to_f64())
                .await?;

            // 4b. Apply new amount to new asset's balance
            let new_asset_sheet = balance_repo
                .get_current_sheet_by_asset_id(user_id, new_asset_id_uuid)
                .await?
                .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for new asset {} not found", new_asset_id_uuid)))?;
            
            let final_new_asset_balance = new_asset_sheet.balance + Decimal::from_f64(new_amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert new amount to Decimal".to_string()))?;
            balance_repo
                .update_current_sheet(user_id, new_asset_id_uuid, final_new_asset_balance.to_f64())
                .await?;
        }

        Ok(updated_transaction)
    }
    
    async fn delete_income_record(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    )
         -> Result<(), RepositoryError>
    {
        let balance_repo = BalanceRepositoryImpl { db_pool: Arc::clone(&self.db_pool) };

        // 1. Fetch the transaction to be deleted to get its amount and asset_id
        let transaction_to_delete = transaction::Entity::find_by_id(transaction_id.as_bytes().to_vec())
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound(format!("Income record {} not found for user {}", transaction_id, user_id)))?;

        let amount_to_subtract = transaction_to_delete.amount;
        let asset_id_uuid = Uuid::from_slice(&transaction_to_delete.asset_id)
            .map_err(|e| RepositoryError::OperationFailed(format!("Invalid asset UUID in transaction to delete: {}", e)))?;

        // 2. Delete the income record
        let delete_result = transaction::Entity::delete_by_id(transaction_id.as_bytes().to_vec())
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if delete_result.rows_affected == 0 {
            // This case should ideally be caught by the find_by_id above, but as a safeguard:
            return Err(RepositoryError::NotFound(format!("Income record {} not found for user {} during delete operation", transaction_id, user_id)));
        }

        // 3. Update the current sheet balance
        let current_sheet = balance_repo.get_current_sheet_by_asset_id(user_id, asset_id_uuid).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet not found for asset {}", asset_id_uuid)))?;
        let new_balance = current_sheet.balance - Decimal::from_f64(amount_to_subtract).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert amount_to_subtract to Decimal".to_string()))?;
        balance_repo.update_current_sheet(user_id, asset_id_uuid, new_balance.to_f64()).await?;
        Ok(())
    }

    
    async fn get_income_record_by_id(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    )
         -> Result<Option<transaction::Model>, RepositoryError>
    {
        // Query the database to find the income record by ID and ensure it belongs to the user
        let income_record = transaction::Entity::find()
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec()))
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the income record if found, or None if not found
        Ok(income_record)
    }

    
    async fn get_all_income_record_by_user(
        &self, 
        user_id: Uuid
    )
         -> Result<Vec<transaction::Model>, RepositoryError>
    {
        // Query the database to retrieve all income records for the given user
        let income_records = transaction::Entity::find()
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of income records
        Ok(income_records)
    }
}
