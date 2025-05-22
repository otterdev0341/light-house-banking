use std::sync::Arc;

use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use uuid::Uuid;

use crate::domain::entities::{asset, contact, transaction_type};
use crate::{
    domain::{dto::transaction_dto::{ReqCreateTransferDto, ReqUpdateTransferDto}, entities::transaction, req_repository::{balance_repository::BalanceRepositoryBase, transaction_repository::TransferRepositoryUtility}},
    infrastructure::database::mysql::impl_repository::balance_repo::BalanceRepositoryImpl, soc::soc_repository::RepositoryError
};





pub struct TransferRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}

impl TransferRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
    
}

#[async_trait::async_trait]
impl TransferRepositoryUtility for TransferRepositoryImpl {
    async fn create_transfer(
        &self,
        user_id: Uuid,
        transfer_dto: ReqCreateTransferDto,
    ) -> Result<transaction::Model, RepositoryError> {
        // Start a transaction
        let txn = self.db_pool.begin().await.map_err(|err| {
            RepositoryError::DatabaseError(format!("Failed to start transaction: {}", err))
        })?;

        // >>>>> Validate the transfer_dto <<<<<
        log::info!("validating transfer_dto: {:?}", transfer_dto);
        // transection type
        // transaction_type 1 : convert to uui
        log::info!("validation transfer_dto.transaction_type_id: {:?}", transfer_dto.transaction_type_id);
        let transaction_type_id_binary = match Uuid::parse_str(&transfer_dto.transaction_type_id) {
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
            log::error!("Transaction type not found for ID: {}", transfer_dto.transaction_type_id);
            return Err(RepositoryError::InvalidInput("Invalid transaction_type_id".to_string()));
        }

        // asset_id
        log::info!("validating transfer_dto.asset_id: {:?}", transfer_dto.asset_id);
        let asset_id_uuid = match Uuid::parse_str(&transfer_dto.asset_id) {
            Ok(uuid) => uuid,
            Err(_) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed("Invalid asset ID UUID".to_string()));
            }
        };
        let asset_id_binary = asset_id_uuid.as_bytes().to_vec();
        let is_asset_id_valid = asset::Entity::find_by_id(asset_id_binary.clone())
            .one(&txn)
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .is_some();
        if !is_asset_id_valid {
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed("Invalid asset ID".to_string()));
        }
        // destination_asset_id
        log::info!("validating transfer_dto.destination_asset_id: {:?}", transfer_dto.destination_asset_id);
        let destination_asset_id_uuid = match Uuid::parse_str(&transfer_dto.destination_asset_id) {
            Ok(uuid) => uuid,
            Err(_) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed("Invalid destination asset ID UUID".to_string()));
            }
        };
        let destination_asset_id_binary = destination_asset_id_uuid.as_bytes().to_vec();
        let is_destination_asset_id_valid = asset::Entity::find_by_id(destination_asset_id_binary.clone())
            .one(&txn)
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .is_some();
        if !is_destination_asset_id_valid {
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed("Invalid destination asset ID".to_string()));
        }
        // contact_id
        log::info!("validating transfer_dto.contact_id: {:?}", transfer_dto.contact_id);
        let contact_id_uuid = match Uuid::parse_str(&transfer_dto.contact_id) {
            Ok(uuid) => uuid,
            Err(_) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed("Invalid contact ID UUID".to_string()));
            }
        };
        let contact_id_binary = contact_id_uuid.as_bytes().to_vec();
        let is_contact_id_valid = contact::Entity::find_by_id(contact_id_binary.clone())
            .one(&txn)
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .is_some();
        if !is_contact_id_valid {
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed("Invalid contact ID".to_string()));
        }
        // >>>>> Validate the transfer_dto <<<<<
        let balance_repo = BalanceRepositoryImpl {
            db_pool: Arc::clone(&self.db_pool),
        };

        // Create the ActiveModel for the transfer transaction
        let new_transfer = transaction::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the transaction
            transaction_type_id: Set(transaction_type_id_binary.clone()),
            amount: Set(transfer_dto.amount),
            asset_id: Set(asset_id_binary.clone()),
            destination_asset_id: Set(Some(destination_asset_id_binary.clone())),
            contact_id: Set(Some(contact_id_binary.clone())),
            note: Set(transfer_dto.note),
            user_id: Set(user_id.as_bytes().to_vec()),
            // created_at and updated_at will be set by default
            ..Default::default()
        };

        // Insert the transfer transaction into the database
        let inserted_transfer = match new_transfer.insert(&txn).await {
            Ok(record) => record,
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::DatabaseError(err.to_string()));
            }
        };

        // Update balances
        let amount = inserted_transfer.amount;
        let source_asset_id_uuid = Uuid::from_slice(&inserted_transfer.asset_id)
            .map_err(|e| RepositoryError::OperationFailed(format!("Invalid source asset UUID: {}", e)))?;
        let dest_asset_id_uuid = Uuid::from_slice(inserted_transfer.destination_asset_id.as_ref()
            .ok_or_else(|| RepositoryError::OperationFailed("Destination asset ID is missing".to_string()))?)
            .map_err(|e| RepositoryError::OperationFailed(format!("Invalid destination asset UUID: {}", e)))?;

        // Decrease balance of source asset
        let source_sheet = match balance_repo
            .get_current_sheet_by_asset_id(user_id, source_asset_id_uuid)
            .await
        {
            Ok(Some(sheet)) => sheet,
            Ok(None) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::NotFound(format!(
                    "Current sheet for source asset {} not found",
                    source_asset_id_uuid
                )));
            }
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(err);
            }
        };

        let new_source_balance = source_sheet.balance - Decimal::from_f64(amount)
            .ok_or_else(|| RepositoryError::OperationFailed("Failed to convert amount to Decimal".to_string()))?;

        if let Err(err) = balance_repo
            .update_current_sheet(user_id, source_asset_id_uuid, Some(new_source_balance.to_f64().unwrap()))
            .await
        {
            txn.rollback().await.ok(); // Rollback on error
            return Err(err);
        }

        // Increase balance of destination asset
        let dest_sheet = match balance_repo
            .get_current_sheet_by_asset_id(user_id, dest_asset_id_uuid)
            .await
        {
            Ok(Some(sheet)) => sheet,
            Ok(None) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::NotFound(format!(
                    "Current sheet for destination asset {} not found",
                    dest_asset_id_uuid
                )));
            }
            Err(err) => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(err);
            }
        };

        let new_dest_balance = dest_sheet.balance + Decimal::from_f64(amount)
            .ok_or_else(|| RepositoryError::OperationFailed("Failed to convert amount to Decimal".to_string()))?;

        if let Err(err) = balance_repo
            .update_current_sheet(user_id, dest_asset_id_uuid, Some(new_dest_balance.to_f64().unwrap()))
            .await
        {
            txn.rollback().await.ok(); // Rollback on error
            return Err(err);
        }

        // Commit the transaction
        txn.commit().await.map_err(|err| {
            RepositoryError::DatabaseError(format!("Failed to commit transaction: {}", err))
        })?;

        Ok(inserted_transfer)
    }

    
    async fn update_transfer(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid, 
        transfer_dto: ReqUpdateTransferDto
    ) -> Result<transaction::Model, RepositoryError> {
        log::info!("Starting update_transfer for user_id: {}, transaction_id: {}", user_id, transaction_id);

        // Start a transaction
        log::debug!("Starting database transaction...");
        let txn = self.db_pool.begin().await.map_err(|err| {
            log::error!("Failed to start transaction: {}", err);
            RepositoryError::DatabaseError(format!("Failed to start transaction: {}", err))
        })?;

        // Validate the transfer_dto
        log::info!("Validating transfer_dto: {:?}", transfer_dto.clone());
        //asset 
        
        let asset_id_binary = match &transfer_dto.asset_id {
            Some(asset_id) => match Uuid::parse_str(&asset_id.clone()) {
                Ok(uuid) => uuid.as_bytes().to_vec(),
                Err(_) => {
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::OperationFailed("Invalid asset ID UUID".to_string()));
                }
            },
            None => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed("Asset ID is missing".to_string()));
            }
        };
        let is_asset_id_valid = asset::Entity::find_by_id(asset_id_binary.clone())
            .one(&txn)
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .is_some();
        if !is_asset_id_valid {
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed("Invalid asset ID".to_string()));
        }

        // dest_assest
        let destination_asset_id_binary = match transfer_dto.destination_asset_id {
            Some(destination_asset_id) => match Uuid::parse_str(&destination_asset_id) {
                Ok(uuid) => uuid.as_bytes().to_vec(),
                Err(_) => {
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::OperationFailed("Invalid destination asset ID UUID".to_string()));
                }
            },
            None => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed("Destination asset ID is missing".to_string()));
            }
        };
        let is_destination_asset_id_valid = asset::Entity::find_by_id(destination_asset_id_binary.clone())
            .one(&txn)
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .is_some();
        if !is_destination_asset_id_valid {
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed("Invalid destination asset ID".to_string()));
        }
        //contact
        let contact_id_binary = match transfer_dto.contact_id {
            Some(contact_id) => match Uuid::parse_str(&contact_id) {
                Ok(uuid) => uuid.as_bytes().to_vec(),
                Err(_) => {
                    txn.rollback().await.ok(); // Rollback on error
                    return Err(RepositoryError::OperationFailed("Invalid contact ID UUID".to_string()));
                }
            },
            None => {
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed("Contact ID is missing".to_string()));
            }
        };
        let is_contact_id_valid = contact::Entity::find_by_id(contact_id_binary.clone())
            .one(&txn)
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .is_some();
        if !is_contact_id_valid {
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed("Invalid contact ID".to_string()));
        }
        

        // end validation



        let balance_repo = BalanceRepositoryImpl { db_pool: Arc::clone(&self.db_pool) };

        // 1. Fetch the original transaction
        log::debug!("Fetching original transaction for transaction_id: {}", transaction_id);
        let original_transaction = transaction::Entity::find_by_id(transaction_id.as_bytes().to_vec())
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(&txn)
            .await
            .map_err(|err| {
                log::error!("Error fetching original transaction: {}", err);
                RepositoryError::DatabaseError(err.to_string())
            })?
            .ok_or_else(|| {
                log::error!("Transaction not found for transaction_id: {}, user_id: {}", transaction_id, user_id);
                RepositoryError::NotFound(format!("Transaction {} not found for user {}", transaction_id, user_id))
            })?;

    let old_amount = original_transaction.amount;
    let old_source_asset_id_uuid = Uuid::from_slice(&original_transaction.asset_id)
        .map_err(|e| RepositoryError::OperationFailed(format!("Invalid old source asset UUID: {}", e)))?;
    let old_dest_asset_id_uuid = Uuid::from_slice(original_transaction.destination_asset_id.as_ref()
        .ok_or_else(|| RepositoryError::OperationFailed("Old destination asset ID is missing".to_string()))?)
        .map_err(|e| RepositoryError::OperationFailed(format!("Invalid old destination asset UUID: {}", e)))?;

    // 2. Prepare and execute the update for the transaction record
    log::debug!("Preparing to update transaction record...");
    let mut active_model: transaction::ActiveModel = original_transaction.into();
    
    if let Some(amount) = transfer_dto.amount {
        active_model.amount = Set(amount);
    }
    if let Some(_asset_id) = &transfer_dto.asset_id {
        active_model.asset_id = Set(asset_id_binary);
    }
    
    active_model.destination_asset_id = Set(Some(destination_asset_id_binary));
    
    
    active_model.contact_id = Set(Some(contact_id_binary));
    
    if let Some(note) = transfer_dto.note {
        active_model.note = Set(note);
    }
    

    let updated_transaction = active_model
        .update(&txn)
        .await
        .map_err(|err| {
            log::error!("Failed to update transaction record: {}", err);
            RepositoryError::DatabaseError(err.to_string())
        })?;
    log::debug!("Transaction record updated successfully: {:?}", updated_transaction);

    // 3. Get new values
    let new_amount = updated_transaction.amount;
    let new_source_asset_id_uuid = Uuid::from_slice(&updated_transaction.asset_id)
        .map_err(|e| RepositoryError::OperationFailed(format!("Invalid new source asset UUID: {}", e)))?;
    let new_dest_asset_id_uuid = Uuid::from_slice(updated_transaction.destination_asset_id.as_ref()
        .ok_or_else(|| RepositoryError::OperationFailed("New destination asset ID is missing".to_string()))?)
        .map_err(|e| RepositoryError::OperationFailed(format!("Invalid new destination asset UUID: {}", e)))?;

    // Ensure destination_asset_id is not the same as asset_id for the new state
    if new_source_asset_id_uuid == new_dest_asset_id_uuid {
        txn.rollback().await.ok(); // Rollback on error
        return Err(RepositoryError::OperationFailed("Source and destination asset cannot be the same after update.".to_string()));
    }

    // 4. Update balances
    log::debug!("Updating balances...");
    // 4a. Revert old amount from old source asset
    let old_source_sheet = balance_repo.get_current_sheet_by_asset_id(user_id, old_source_asset_id_uuid).await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for old source asset {} not found", old_source_asset_id_uuid)))?;
    let bal_after_reverting_old_source = old_source_sheet.balance + Decimal::from_f64(old_amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert old_amount to Decimal".to_string()))?;
    balance_repo.update_current_sheet(user_id, old_source_asset_id_uuid, Some(bal_after_reverting_old_source.to_f64().unwrap())).await?;

    // 4b. Revert old amount from old destination asset
    let old_dest_sheet = balance_repo.get_current_sheet_by_asset_id(user_id, old_dest_asset_id_uuid).await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for old destination asset {} not found", old_dest_asset_id_uuid)))?;
    let bal_after_reverting_old_dest = old_dest_sheet.balance - Decimal::from_f64(old_amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert old_amount to Decimal".to_string()))?;
    balance_repo.update_current_sheet(user_id, old_dest_asset_id_uuid, Some(bal_after_reverting_old_dest.to_f64().unwrap())).await?;

    // 4c. Apply new amount to new source asset
    let new_source_sheet = balance_repo.get_current_sheet_by_asset_id(user_id, new_source_asset_id_uuid).await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for new source asset {} not found", new_source_asset_id_uuid)))?;
    let final_new_source_balance = new_source_sheet.balance - Decimal::from_f64(new_amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert new_amount to Decimal".to_string()))?;
    balance_repo.update_current_sheet(user_id, new_source_asset_id_uuid, Some(final_new_source_balance.to_f64().unwrap())).await?;

    // 4d. Apply new amount to new destination asset
    let new_dest_sheet = balance_repo.get_current_sheet_by_asset_id(user_id, new_dest_asset_id_uuid).await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for new destination asset {} not found", new_dest_asset_id_uuid)))?;
    let final_new_dest_balance = new_dest_sheet.balance + Decimal::from_f64(new_amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert new_amount to Decimal".to_string()))?;
    balance_repo.update_current_sheet(user_id, new_dest_asset_id_uuid, Some(final_new_dest_balance.to_f64().unwrap())).await?;

    // Commit the transaction
    log::debug!("Committing transaction...");
    txn.commit().await.map_err(|err| {
        log::error!("Failed to commit transaction: {}", err);
        RepositoryError::DatabaseError(format!("Failed to commit transaction: {}", err))
    })?;
    log::info!("Transaction committed successfully for update_transfer.");

    Ok(updated_transaction)
}
    
    async fn delete_transfer(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    ) 
        -> Result<(), RepositoryError> 
    {
        let balance_repo = BalanceRepositoryImpl { db_pool: Arc::clone(&self.db_pool) };


        // 1. Fetch the transaction to be deleted
        let transaction_to_delete = transaction::Entity::find_by_id(transaction_id.as_bytes().to_vec())
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound(format!("Transaction {} not found for user {}", transaction_id, user_id)))?;

        let amount = transaction_to_delete.amount;
        let source_asset_id_uuid = Uuid::from_slice(&transaction_to_delete.asset_id)
            .map_err(|e| RepositoryError::OperationFailed(format!("Invalid source asset UUID in transaction to delete: {}", e)))?;
        let dest_asset_id_uuid = Uuid::from_slice(transaction_to_delete.destination_asset_id.as_ref()
            .ok_or_else(|| RepositoryError::OperationFailed("Destination asset ID is missing in transaction to delete".to_string()))?)
            .map_err(|e| RepositoryError::OperationFailed(format!("Invalid destination asset UUID in transaction to delete: {}", e)))?;

        // 2. Delete the transaction record
        let delete_result = transaction::Entity::delete_by_id(transaction_id.as_bytes().to_vec())
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if delete_result.rows_affected == 0 {
            return Err(RepositoryError::NotFound(format!("Transaction {} not found for user {} during delete operation", transaction_id, user_id)));
        }

        // 3. Update balances
        // Add amount back to source asset
        let source_sheet = balance_repo.get_current_sheet_by_asset_id(user_id, source_asset_id_uuid).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for source asset {} not found", source_asset_id_uuid)))?;
        let new_source_balance = source_sheet.balance + Decimal::from_f64(amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert amount to Decimal".to_string()))?;
        balance_repo.update_current_sheet(user_id, source_asset_id_uuid, new_source_balance.to_f64()).await?;

        // Subtract amount from destination asset
        let dest_sheet = balance_repo.get_current_sheet_by_asset_id(user_id, dest_asset_id_uuid).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Current sheet for destination asset {} not found", dest_asset_id_uuid)))?;
        let new_dest_balance = dest_sheet.balance - Decimal::from_f64(amount).ok_or_else(|| RepositoryError::OperationFailed("Failed to convert amount to Decimal".to_string()))?;
        balance_repo.update_current_sheet(user_id, dest_asset_id_uuid, new_dest_balance.to_f64()).await?;

        Ok(())
    }

    
    async fn get_transfer_by_id(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    ) 
        -> Result<Option<transaction::Model>, RepositoryError> 
    {
        let transaction_id = Uuid::from_str(&transaction_id.to_string())
            .map_err(|_| RepositoryError::OperationFailed("Invalid transaction ID".to_string()))?;
        let transction_id_binary = transaction_id.as_bytes().to_vec();
        // Check if the transaction ID is valid
        // Query the database to find the transaction by ID and ensure it belongs to the user
        let transaction = transaction::Entity::find()
            .filter(transaction::Column::Id.eq(transction_id_binary)) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the transaction if found, or None if not found
        Ok(transaction)
    }

    
    async fn get_all_transfers_by_user(
        &self, 
        user_id: Uuid
    ) 
        -> Result<Vec<transaction::Model>, RepositoryError> 
    {
        let transfer_model = transaction_type::Entity::find()
            .filter(transaction_type::Column::Name.eq("transfer"))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        let transfer_uuid = match transfer_model {
            Some(model) => match Uuid::from_slice(&model.id) {
                Ok(uuid) => uuid,
                Err(err) => {
                    return Err(RepositoryError::DatabaseError(format!(
                        "Failed to parse UUID: {}",
                        err
                    )));
                }
            },
            None => {
                return Err(RepositoryError::NotFound("Income transaction type not found".to_string()));
            }
        };

        // Query the database to retrieve all transactions for the given user
        let transactions = transaction::Entity::find()
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .filter(transaction::Column::TransactionTypeId.eq(transfer_uuid.as_bytes().to_vec())) // Filter by transaction type
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of transactions
        Ok(transactions)
    }
}