use std::sync::Arc;

use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use crate::domain::entities::{asset, contact, transaction_type};
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
        // validation fk 
        // Check if the transaction type ID is valid
        let transaction_type_id_binary = match Uuid::parse_str(&income_record_dto.transaction_type_id) {
            Ok(uuid) => uuid.as_bytes().to_vec(),
            Err(err) => {
                log::error!("Invalid transaction_type_id: {}", err);
                return Err(RepositoryError::InvalidInput("Invalid transaction_type_id".to_string()));
            }
        };
        
        let is_transaction_type_valid = transaction_type::Entity::find_by_id(transaction_type_id_binary.clone())
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        if is_transaction_type_valid.is_none() {
            log::error!("Transaction type not found for ID: {}", income_record_dto.transaction_type_id);
            return Err(RepositoryError::InvalidInput("Invalid transaction_type_id".to_string()));
        }

        // Check if the asset ID is valid
        let asset_id_binary = match Uuid::parse_str(&income_record_dto.asset_id) {
            Ok(uuid) => uuid.as_bytes().to_vec(),
            Err(err) => {
                log::error!("Invalid asset_type_id: {}", err);
                return Err(RepositoryError::InvalidInput("Invalid transaction_type_id".to_string()));
            }
        };
        
        let is_asset_valid = asset::Entity::find_by_id(asset_id_binary.clone())
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        if is_asset_valid.is_none() {
            log::error!("Asset not found for ID: {}", income_record_dto.asset_id);
            return Err(RepositoryError::InvalidInput("Invalid asset_id".to_string()));
        }
        // Check if the contact ID is valid
        let contact_id_binary = match Uuid::parse_str(&income_record_dto.contact_id) {
            Ok(uuid) => uuid.as_bytes().to_vec(),
            Err(err) => {
                log::error!("Invalid contact_id: {}", err);
                return Err(RepositoryError::InvalidInput("Invalid transaction_type_id".to_string()));
            }
        };
        let is_contact_valid = contact::Entity::find_by_id(contact_id_binary.clone())
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        if is_contact_valid.is_none() && is_contact_valid.is_none() {
            log::error!("Asset not found for ID: {}", income_record_dto.contact_id);
            return Err(RepositoryError::InvalidInput("Invalid contact_id".to_string()));
        }
  


        // 1: start transaction
        let txn = self.db_pool.begin().await.map_err(|err| {
            RepositoryError::DatabaseError(format!("Failed to start transaction: {}", err))
        })?;

        // 2 initial balance repo to
        let balance_repo = BalanceRepositoryImpl {
            db_pool: Arc::clone(&self.db_pool),
        };
        log::info!("Creating income record for user: {}", user_id);
        // Create the ActiveModel for the income record
        let new_income_record = transaction::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the transaction
            transaction_type_id: Set(transaction_type_id_binary),
            amount: Set(income_record_dto.amount),
            asset_id: Set(asset_id_binary),
            contact_id: Set(Some(contact_id_binary)),
            note: Set(income_record_dto.note),
            user_id: Set(user_id.as_bytes().to_vec()),
            ..Default::default()
        };

        // Insert the income record into the database
        let inserted_income_record = match new_income_record.insert(&txn).await {
            Ok(record) => record,
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::DatabaseError(err.to_string()));
            }
        };

        // Update the balance in the CurrentSheet table
        let asset_id_uuid = match Uuid::from_slice(&inserted_income_record.asset_id) {
            Ok(uuid) => uuid,
            Err(e) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid asset UUID: {}",
                    e
                )));
            }
        };

        let current_sheet = match balance_repo
            .get_current_sheet_by_asset_id(user_id, asset_id_uuid)
            .await
            {
            Ok(Some(sheet)) => sheet,
            Ok(None) => {
                log::error!("Current sheet not found for asset ID: {}", asset_id_uuid);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::NotFound(format!(
                    "Current sheet not found for asset {}",
                    asset_id_uuid
                )));
            }
            Err(err) => {
                log::error!("Error fetching current sheet: {}", err);
                txn.rollback().await.ok(); // Rollback on error
                return Err(err);
            }
        };

        let the_new_balance = match Decimal::from_f64(inserted_income_record.amount) {
            Some(amount) => {
                log::info!("Amount from dto to add: {}", amount);
                log::info!("Amount from current table : {} + value from database {}", current_sheet.balance, amount);
                amount + current_sheet.balance
            },
            None => {
                log::error!("Failed to convert amount to Decimal: {}", inserted_income_record.amount);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(
                    "Failed to convert amount to Decimal".to_string(),
                ));
            }
        };

       

        // Round the new balance to 2 decimal places
        let rounded_balance = the_new_balance.round_dp(2);
        log::info!("Rounded balance: {}", rounded_balance);

        if let Err(err) = balance_repo
            .update_current_sheet(user_id, asset_id_uuid, Some(rounded_balance.to_f64().unwrap()))
            .await
        {
            log::error!("Failed to update current sheet balance: {}", err);
            txn.rollback().await.ok(); // Rollback on error
            return Err(err);
        }

        txn.commit().await.map_err(|err| {
            log::error!("Failed to commit transaction: {}", err);
            RepositoryError::DatabaseError(format!("Failed to commit transaction: {}", err))
        })?;

        Ok(inserted_income_record)
    }

    
    async fn update_income_record(
        &self,
        user_id: Uuid,
        transaction_id: Uuid,
        income_record_dto: ReqUpdateIncomeDto,
    ) -> Result<transaction::Model, RepositoryError> {
        // Start a transaction
        let txn = self.db_pool.begin().await.map_err(|err| {
            RepositoryError::DatabaseError(format!("Failed to start transaction: {}", err))
        })?;
    
        let balance_repo = BalanceRepositoryImpl {
            db_pool: Arc::clone(&self.db_pool),
        };
    
        // Fetch the original transaction
        let original_transaction = match transaction::Entity::find_by_id(transaction_id.as_bytes().to_vec())
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(&txn)
            .await
        {
            Ok(Some(transaction)) => transaction,
            Ok(None) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::NotFound(format!(
                    "Transaction {} not found for user {}",
                    transaction_id, user_id
                )));
            }
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::DatabaseError(err.to_string()));
            }
        };
    
        let old_amount = original_transaction.amount;
        let old_asset_id_uuid = match Uuid::from_slice(&original_transaction.asset_id) {
            Ok(uuid) => uuid,
            Err(e) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid old asset UUID: {}",
                    e
                )));
            }
        };
    
        // Validate and parse the new asset ID
        let new_asset_id_binary = match income_record_dto.asset_id.as_ref() {
            Some(asset_id) => match Uuid::parse_str(asset_id) {
                Ok(uuid) => uuid.as_bytes().to_vec(),
                Err(err) => {
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::InvalidInput(format!(
                        "Invalid asset_id: {}",
                        err
                    )));
                }
            },
            None => original_transaction.asset_id.clone(),
        };
    
        // Validate the new asset ID
        let is_asset_valid = asset::Entity::find_by_id(new_asset_id_binary.clone())
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        if is_asset_valid.is_none() {
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::InvalidInput("Invalid asset_id".to_string()));
        }
    
        // Update the transaction
        let mut active_model: transaction::ActiveModel = original_transaction.into();
        if let Some(amount) = income_record_dto.amount {
            active_model.amount = Set(amount);
        }
        if let Some(_asset_id) = income_record_dto.asset_id {
            active_model.asset_id = Set(new_asset_id_binary.clone());
        }
        if let Some(contact_id) = income_record_dto.contact_id {
            let contact_id_binary = match Uuid::parse_str(&contact_id) {
                Ok(uuid) => uuid.as_bytes().to_vec(),
                Err(err) => {
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::InvalidInput(format!(
                        "Invalid contact_id: {}",
                        err
                    )));
                }
            };
    
            // Validate the contact ID
            let is_contact_valid = contact::Entity::find_by_id(contact_id_binary.clone())
                .one(self.db_pool.as_ref())
                .await
                .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
            if is_contact_valid.is_none() {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::InvalidInput("Invalid contact_id".to_string()));
            }
    
            active_model.contact_id = Set(Some(contact_id_binary));
        }
        if let Some(note) = income_record_dto.note {
            active_model.note = Set(note);
        }
    
        let updated_transaction = match active_model.update(&txn).await {
            Ok(transaction) => transaction,
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::DatabaseError(err.to_string()));
            }
        };
    
        // Update balances
        let new_amount = updated_transaction.amount;
        let new_asset_id_uuid = match Uuid::from_slice(&updated_transaction.asset_id) {
            Ok(uuid) => uuid,
            Err(e) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid new asset UUID: {}",
                    e
                )));
            }
        };
    
        log::debug!("Old amount: {}", old_amount);
        log::debug!("New amount: {}", new_amount);
        log::debug!("Old asset ID: {}", old_asset_id_uuid);
        log::debug!("New asset ID: {}", new_asset_id_uuid);
    
        if old_asset_id_uuid == new_asset_id_uuid {
            // Asset ID hasn't changed, calculate net change
            let balance_change = new_amount - old_amount;
            log::debug!("Balance change: {}", balance_change);
    
            let current_sheet = match balance_repo
                .get_current_sheet_by_asset_id(user_id, new_asset_id_uuid)
                .await
            {
                Ok(Some(sheet)) => sheet,
                Ok(None) => {
                    log::error!("Current sheet not found for asset ID: {}", new_asset_id_uuid);
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::NotFound(format!(
                        "Current sheet for asset {} not found",
                        new_asset_id_uuid
                    )));
                }
                Err(err) => {
                    log::error!("Error fetching current sheet: {}", err);
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(err);
                }
            };
    
            let final_balance = match Decimal::from_f64(balance_change) {
                Some(change) => current_sheet.balance + change,
                None => {
                    log::error!("Failed to convert balance change to Decimal");
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::OperationFailed(
                        "Failed to convert balance change to Decimal".to_string(),
                    ));
                }
            };
            log::debug!("Final balance: {}", final_balance);
    
            if let Err(err) = balance_repo
                .update_current_sheet(user_id, new_asset_id_uuid, Some(final_balance.to_f64().unwrap()))
                .await
            {
                log::error!("Failed to update current sheet balance: {}", err);
                txn.rollback().await.ok(); // Rollback on error
                return Err(err);
            }
        } else {
            // Handle case where asset ID has changed
            let old_current_sheet = match balance_repo
                .get_current_sheet_by_asset_id(user_id, old_asset_id_uuid)
                .await
            {
                Ok(Some(sheet)) => sheet,
                Ok(None) => {
                    log::error!("Current sheet not found for old asset ID: {}", old_asset_id_uuid);
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::NotFound(format!(
                        "Current sheet for old asset {} not found",
                        old_asset_id_uuid
                    )));
                }
                Err(err) => {
                    log::error!("Error fetching current sheet for old asset: {}", err);
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(err);
                }
            };
    
            let new_current_sheet = match balance_repo
                .get_current_sheet_by_asset_id(user_id, new_asset_id_uuid)
                .await
            {
                Ok(Some(sheet)) => sheet,
                Ok(None) => {
                    log::error!("Current sheet not found for new asset ID: {}", new_asset_id_uuid);
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::NotFound(format!(
                        "Current sheet for new asset {} not found",
                        new_asset_id_uuid
                    )));
                }
                Err(err) => {
                    log::error!("Error fetching current sheet for new asset: {}", err);
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(err);
                }
            };
    
            // Adjust balances for both old and new assets
            let old_balance = old_current_sheet.balance - Decimal::from_f64(old_amount).ok_or_else(|| {
                RepositoryError::OperationFailed("Failed to convert old_amount to Decimal".to_string())
            })?;
            let new_balance = new_current_sheet.balance + Decimal::from_f64(new_amount).ok_or_else(|| {
                RepositoryError::OperationFailed("Failed to convert new_amount to Decimal".to_string())
            })?;
    
            if let Err(err) = balance_repo
                .update_current_sheet(user_id, old_asset_id_uuid, Some(old_balance.to_f64().unwrap()))
                .await
            {
                log::error!("Failed to update old current sheet balance: {}", err);
                txn.rollback().await.ok(); // Rollback on error
                return Err(err);
            }
    
            if let Err(err) = balance_repo
                .update_current_sheet(user_id, new_asset_id_uuid, Some(new_balance.to_f64().unwrap()))
                .await
            {
                log::error!("Failed to update new current sheet balance: {}", err);
                txn.rollback().await.ok(); // Rollback on error
                return Err(err);
            }
        }
    
        // Commit the transaction
        txn.commit().await.map_err(|err| {
            RepositoryError::DatabaseError(format!("Failed to commit transaction: {}", err))
        })?;
    
        Ok(updated_transaction)
    }
    




    async fn delete_income_record(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    )
         -> Result<(), RepositoryError>
    {
        // Start a transaction
        let txn = self.db_pool.begin().await.map_err(|err| {
            RepositoryError::DatabaseError(format!("Failed to start transaction: {}", err))
        })?;

        let balance_repo = BalanceRepositoryImpl {
            db_pool: Arc::clone(&self.db_pool),
        };

        // Fetch the transaction to be deleted
        let transaction_to_delete = match transaction::Entity::find_by_id(transaction_id.as_bytes().to_vec())
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(&txn)
            .await
        {
            Ok(Some(transaction)) => transaction,
            Ok(None) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::NotFound(format!(
                    "Income record {} not found for user {}",
                    transaction_id, user_id
                )));
            }
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::DatabaseError(err.to_string()));
            }
        };

        let amount_to_subtract = transaction_to_delete.amount;
        let asset_id_uuid = match Uuid::from_slice(&transaction_to_delete.asset_id) {
            Ok(uuid) => uuid,
            Err(e) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid asset UUID: {}",
                    e
                )));
            }
        };

        // Delete the income record
        let delete_result = match transaction::Entity::delete_by_id(transaction_id.as_bytes().to_vec())
            .exec(&txn)
            .await
        {
            Ok(result) => result,
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::DatabaseError(err.to_string()));
            }
        };

        if delete_result.rows_affected == 0 {
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::NotFound(format!(
                "Income record {} not found for user {} during delete operation",
                transaction_id, user_id
            )));
        }

        // Update the balance in the CurrentSheet table
        let current_sheet = match balance_repo
            .get_current_sheet_by_asset_id(user_id, asset_id_uuid)
            .await
        {
            Ok(Some(sheet)) => sheet,
            Ok(None) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::NotFound(format!(
                    "Current sheet not found for asset {}",
                    asset_id_uuid
                )));
            }
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(err);
            }
        };

        let new_balance = match Decimal::from_f64(amount_to_subtract) {
            Some(amount) => current_sheet.balance - amount,
            None => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(
                    "Failed to convert amount_to_subtract to Decimal".to_string(),
                ));
            }
        };

        if let Err(err) = balance_repo
            .update_current_sheet(user_id, asset_id_uuid, Some(new_balance.to_f64().unwrap()))
            .await
        {
            txn.rollback().await.ok(); // Rollback on error
            return Err(err);
        }

        // Commit the transaction
        txn.commit().await.map_err(|err| {
            RepositoryError::DatabaseError(format!("Failed to commit transaction: {}", err))
        })?;

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
