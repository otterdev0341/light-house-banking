use std::sync::Arc;
#[allow(unused_imports)]
use rust_decimal_macros::*;
use rust_decimal::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait};
use uuid::Uuid;

use crate::{domain::{dto::transaction_dto::{ReqCreatePaymentDto, ReqUpdatePaymentDto}, entities::{asset, contact, expense, transaction, transaction_type}, req_repository::{balance_repository::BalanceRepositoryBase, transaction_repository::RecordPaymentRepositoryUtility}}, infrastructure::database::mysql::impl_repository::balance_repo::BalanceRepositoryImpl, soc::soc_repository::RepositoryError};





pub struct PaymentRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}

impl PaymentRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait] 
impl RecordPaymentRepositoryUtility for PaymentRepositoryImpl {


    async fn create_payment_record(
        &self, 
        user_id: Uuid, 
        payment_record_dto: ReqCreatePaymentDto
    ) 
        -> Result<transaction::Model, RepositoryError>
    {
        log::info!("Starting create_payment_record for user_id: {}", user_id);

        // Start a transaction
        log::debug!("Starting database transaction...");
        let txn = self.db_pool.begin().await.map_err(|err| {
            log::error!("Failed to start transaction: {}", err);
            RepositoryError::DatabaseError(format!("Failed to start transaction: {}", err))
        })?;

        // Validate transaction_type_id
        log::debug!("Validating transaction_type_id: {}", payment_record_dto.transaction_type_id);
        let transaction_type_id_binary = match Uuid::parse_str(&payment_record_dto.transaction_type_id) {
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
            log::error!("Transaction type not found for ID: {}", payment_record_dto.transaction_type_id);
            return Err(RepositoryError::InvalidInput("Invalid transaction_type_id".to_string()));
        }

        // Validate expense_id
        log::debug!("Validating expense_id: {}", payment_record_dto.expense_id);
        let expense_id_binary = match Uuid::parse_str(&payment_record_dto.expense_id) {
            Ok(uuid) => uuid.as_bytes().to_vec(),
            Err(e) => {
                log::error!("Invalid expense_id: {}", e);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid expense UUID: {}",
                    e
                )));
            }
        };

        let is_expense_valid = expense::Entity::find_by_id(expense_id_binary.clone())
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                log::error!("Error querying expense_id: {}", err);
                RepositoryError::DatabaseError(err.to_string())
            })?;
        if is_expense_valid.is_none() {
            log::error!("Expense not found for ID: {}", payment_record_dto.expense_id);
            return Err(RepositoryError::InvalidInput("Invalid expense id".to_string()));
        }

        // Validate asset_id
        log::debug!("Validating asset_id: {}", payment_record_dto.asset_id);
        let asset_id_binary = match Uuid::parse_str(&payment_record_dto.asset_id) {
            Ok(uuid) => uuid.as_bytes().to_vec(),
            Err(e) => {
                log::error!("Invalid asset_id: {}", e);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid asset UUID: {}",
                    e
                )));
            }
        };

        let is_asset_valid = asset::Entity::find_by_id(asset_id_binary.clone())
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                log::error!("Error querying asset_id: {}", err);
                RepositoryError::DatabaseError(err.to_string())
            })?;
        if is_asset_valid.is_none() {
            log::error!("Asset not found for ID: {}", payment_record_dto.asset_id);
            return Err(RepositoryError::InvalidInput("Invalid asset id".to_string()));
        }

        // Validate contact_id
        log::debug!("Validating contact_id: {}", payment_record_dto.contact_id);
        let contact_id_binary = match Uuid::parse_str(&payment_record_dto.contact_id) {
            Ok(uuid) => uuid.as_bytes().to_vec(),
            Err(e) => {
                log::error!("Invalid contact_id: {}", e);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid contact UUID: {}",
                    e
                )));
            }
        };

        let is_contact_valid = contact::Entity::find_by_id(contact_id_binary.clone())
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                log::error!("Error querying contact_id: {}", err);
                RepositoryError::DatabaseError(err.to_string())
            })?;
        if is_contact_valid.is_none() {
            log::error!("Contact not found for ID: {}", payment_record_dto.contact_id);
            return Err(RepositoryError::InvalidInput("Invalid contact id".to_string()));
        }

        // Create the ActiveModel for the payment record
        log::debug!("Creating ActiveModel for payment record...");
        let new_payment_record = transaction::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the transaction
            transaction_type_id: Set(transaction_type_id_binary),
            amount: Set(payment_record_dto.amount),
            expense_id: Set(Some(expense_id_binary)),
            asset_id: Set(asset_id_binary),
            contact_id: Set(Some(contact_id_binary)),
            note: Set(payment_record_dto.note),
            user_id: Set(user_id.as_bytes().to_vec()),
            ..Default::default()
        };

        // Insert the payment record into the database
        log::debug!("Inserting payment record into the database...");
        let inserted_payment_record = match new_payment_record.insert(&txn).await {
            Ok(record) => record,
            Err(err) => {
                log::error!("Failed to insert payment record: {}", err);
                txn.rollback().await.ok(); // Rollback on error
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("FOREIGN KEY") {
                        return Err(RepositoryError::OperationFailed(
                            "Invalid foreign key reference in the payment record".to_string(),
                        ));
                    }
                }
                return Err(RepositoryError::DatabaseError(err.to_string()));
            }
        };

        // Update the balance in the CurrentSheet table
        log::debug!("Updating balance in the CurrentSheet table...");
        let asset_id_uuid = match Uuid::from_slice(&inserted_payment_record.asset_id) {
            Ok(uuid) => uuid,
            Err(e) => {
                log::error!("Invalid asset UUID: {}", e);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid asset UUID: {}",
                    e
                )));
            }
        };
        let balance_repo = BalanceRepositoryImpl {
            db_pool: Arc::clone(&self.db_pool),
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

        let new_balance = match Decimal::from_f64(inserted_payment_record.amount) {
            Some(amount) => current_sheet.balance - amount,
            None => {
                log::error!("Failed to convert amount to Decimal: {}", inserted_payment_record.amount);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(
                    "Failed to convert amount to Decimal".to_string(),
                ));
            }
        };

        log::debug!("New balance calculated: {}", new_balance);

        if let Err(err) = balance_repo
            .update_current_sheet(user_id, asset_id_uuid, Some(new_balance.to_f64().unwrap()))
            .await
        {
            log::error!("Failed to update balance in CurrentSheet table: {}", err);
            txn.rollback().await.ok(); // Rollback on error
            return Err(err);
        }

        // Commit the transaction
        log::debug!("Committing transaction...");
        txn.commit().await.map_err(|err| {
            log::error!("Failed to commit transaction: {}", err);
            RepositoryError::DatabaseError(format!("Failed to commit transaction: {}", err))
        })?;

        log::info!("Payment record created successfully for user_id: {}", user_id);
        Ok(inserted_payment_record)
    }


    async fn update_payment_record(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid, 
        payment_record_dto: ReqUpdatePaymentDto
    ) -> Result<transaction::Model, RepositoryError> {
    log::info!("Starting update_payment_record for user_id: {}, transaction_id: {}", user_id, transaction_id);

    // Start a transaction
    log::debug!("Starting database transaction...");
    let txn = self.db_pool.begin().await.map_err(|err| {
        log::error!("Failed to start transaction: {}", err);
        RepositoryError::DatabaseError(format!("Failed to start transaction: {}", err))
    })?;

    let balance_repo = BalanceRepositoryImpl {
        db_pool: Arc::clone(&self.db_pool),
    };

    // Fetch the original transaction to get old_amount and old_asset_id
    log::debug!("Fetching original transaction for transaction_id: {}", transaction_id);
    let original_transaction = match transaction::Entity::find_by_id(transaction_id.as_bytes().to_vec())
        .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
        .one(&txn)
        .await
    {
        Ok(Some(transaction)) => transaction,
        Ok(None) => {
            log::error!("Transaction not found for transaction_id: {}, user_id: {}", transaction_id, user_id);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::NotFound(format!(
                "Transaction {} not found for user {}",
                transaction_id, user_id
            )));
        }
        Err(err) => {
            log::error!("Error fetching original transaction: {}", err);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::DatabaseError(err.to_string()));
        }
    };

    let old_amount = original_transaction.amount;
    log::debug!("Old amount: {}", old_amount);

    let old_asset_id_uuid = match Uuid::from_slice(&original_transaction.asset_id) {
        Ok(uuid) => uuid,
        Err(e) => {
            log::error!("Invalid old asset UUID: {}", e);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed(format!(
                "Invalid old asset UUID: {}",
                e
            )));
        }
    };
    log::debug!("Old asset ID: {}", old_asset_id_uuid);

    // Convert the existing transaction into an ActiveModel for updating
    log::debug!("Converting original transaction into ActiveModel...");
    let mut active_model: transaction::ActiveModel = original_transaction.into();

    // Update fields if they are provided in the DTO
    if let Some(amount) = payment_record_dto.amount {
        log::debug!("Updating amount to: {}", amount);
        active_model.amount = Set(amount);
    }
    if let Some(expense_id) = payment_record_dto.expense_id {
        log::debug!("Updating expense_id to: {}", expense_id);
        let expense_id_binary = match Uuid::parse_str(&expense_id) {
            Ok(uuid) => {
                log::debug!("Parsed expense_id to binary: {:?}", uuid.as_bytes());
                uuid.as_bytes().to_vec()
            }
            Err(e) => {
                log::error!("Invalid expense_id: {}", e);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid expense UUID: {}",
                    e
                )));
            }
        };
        log::debug!("Binary expense_id length: {}", expense_id_binary.len());
        active_model.expense_id = Set(Some(expense_id_binary));
    }
    if let Some(contact_id) = payment_record_dto.contact_id {
        log::debug!("Updating contact_id to: {}", contact_id);
        let contact_id_binary = match Uuid::parse_str(&contact_id) {
            Ok(uuid) => {
                log::debug!("Parsed contact_id to binary: {:?}", uuid.as_bytes());
                uuid.as_bytes().to_vec()
            }
            Err(e) => {
                log::error!("Invalid contact_id: {}", e);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid contact UUID: {}",
                    e
                )));
            }
        };
        active_model.contact_id = Set(Some(contact_id_binary));
    }
    if let Some(note) = payment_record_dto.note {
        log::debug!("Updating note to: {}", note);
        active_model.note = Set(note);
    }
    if let Some(asset_id) = payment_record_dto.asset_id {
        log::debug!("Updating asset_id to: {}", asset_id);
        let asset_id_binary = match Uuid::parse_str(&asset_id) {
            Ok(uuid) => {
                log::debug!("Parsed asset_id to binary: {:?}", uuid.as_bytes());
                uuid.as_bytes().to_vec()
            }
            Err(e) => {
                log::error!("Invalid asset_id: {}", e);
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(format!(
                    "Invalid asset UUID: {}",
                    e
                )));
            }
        };
        active_model.asset_id = Set(asset_id_binary);
    }

    // Save the updated transaction to the database
    log::debug!("Saving updated transaction to the database...");
    let updated_transaction = match active_model.update(&txn).await {
        Ok(transaction) => transaction,
        Err(err) => {
            log::error!("Failed to update transaction: {}", err);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::DatabaseError(err.to_string()));
        }
    };
    log::debug!("Updated transaction saved successfully.");

    // Update balances
    let new_amount = updated_transaction.amount;
    log::debug!("New amount: {}", new_amount);

    let new_asset_id_uuid = match Uuid::from_slice(&updated_transaction.asset_id) {
        Ok(uuid) => uuid,
        Err(e) => {
            log::error!("Invalid new asset UUID: {}", e);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed(format!(
                "Invalid new asset UUID: {}",
                e
            )));
        }
    };
    log::debug!("New asset ID: {}", new_asset_id_uuid);

    if old_asset_id_uuid == new_asset_id_uuid {
        log::debug!("Asset ID has not changed. Calculating balance change...");
        let balance_change = old_amount - new_amount;
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
                log::error!("Failed to convert balance change to Decimal.");
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
        log::debug!("Balance updated successfully for unchanged asset ID.");
    } else {
        log::debug!("Asset ID has changed. Updating balances for old and new assets...");

        // Revert old amount from old asset's balance
        let old_asset_sheet = match balance_repo
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

        let balance_after_reverting_old = match Decimal::from_f64(old_amount) {
            Some(amount) => old_asset_sheet.balance + amount,
            None => {
                log::error!("Failed to convert old_amount to Decimal.");
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(
                    "Failed to convert old_amount to Decimal".to_string(),
                ));
            }
        };
        log::debug!("Balance after reverting old amount: {}", balance_after_reverting_old);

        if let Err(err) = balance_repo
            .update_current_sheet(user_id, old_asset_id_uuid, Some(balance_after_reverting_old.to_f64().unwrap()))
            .await
        {
            log::error!("Failed to update balance for old asset: {}", err);
            txn.rollback().await.ok(); // Rollback on error
            return Err(err);
        }
        log::debug!("Balance updated successfully for old asset.");

        // Apply new amount to new asset's balance
        let new_asset_sheet = match balance_repo
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

        let final_new_asset_balance = match Decimal::from_f64(new_amount) {
            Some(amount) => new_asset_sheet.balance - amount,
            None => {
                log::error!("Failed to convert new amount to Decimal.");
                txn.rollback().await.ok(); // Rollback on error
                return Err(RepositoryError::OperationFailed(
                    "Failed to convert new amount to Decimal".to_string(),
                ));
            }
        };
        log::debug!("Final balance for new asset: {}", final_new_asset_balance);

        if let Err(err) = balance_repo
            .update_current_sheet(user_id, new_asset_id_uuid, Some(final_new_asset_balance.to_f64().unwrap()))
            .await
        {
            log::error!("Failed to update balance for new asset: {}", err);
            txn.rollback().await.ok(); // Rollback on error
            return Err(err);
        }
        log::debug!("Balance updated successfully for new asset.");
    }

    // Commit the transaction
    log::debug!("Committing transaction...");
    txn.commit().await.map_err(|err| {
        log::error!("Failed to commit transaction: {}", err);
        RepositoryError::DatabaseError(format!("Failed to commit transaction: {}", err))
    })?;
    log::info!("Transaction committed successfully for transaction_id: {}", transaction_id);

    Ok(updated_transaction)
}


    async fn delete_payment_record(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    ) -> Result<(), RepositoryError> {
    log::info!("Starting delete_payment_record for user_id: {}, transaction_id: {}", user_id, transaction_id);

    // Start a transaction
    log::debug!("Starting database transaction...");
    let txn = self.db_pool.begin().await.map_err(|err| {
        log::error!("Failed to start transaction: {}", err);
        RepositoryError::DatabaseError(format!("Failed to start transaction: {}", err))
    })?;

    let balance_repo = BalanceRepositoryImpl {
        db_pool: Arc::clone(&self.db_pool),
    };

    // Validate transaction_id
    let transaction_id_binary = match Uuid::parse_str(&transaction_id.to_string()) {
        Ok(uuid) => uuid.as_bytes().to_vec(),
        Err(err) => {
            log::error!("Invalid transaction_id: {}", err);
            return Err(RepositoryError::InvalidInput("Invalid transaction_id".to_string()));
        }
    };

    // Fetch the transaction to be deleted
    log::debug!("Fetching transaction to delete for transaction_id: {}", transaction_id);
    let transaction_to_delete = match transaction::Entity::find_by_id(transaction_id_binary)
        .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
        .one(&txn)
        .await
    {
        Ok(Some(transaction)) => transaction,
        Ok(None) => {
            log::error!("Transaction not found for transaction_id: {}, user_id: {}", transaction_id, user_id);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::NotFound(format!(
                "Payment record {} not found for user {}",
                transaction_id, user_id
            )));
        }
        Err(err) => {
            log::error!("Error fetching transaction to delete: {}", err);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::DatabaseError(err.to_string()));
        }
    };

    log::debug!("Transaction to delete found: {:?}", transaction_to_delete);

    let amount_to_add_back = transaction_to_delete.amount;
    log::debug!("Amount to add back to balance: {}", amount_to_add_back);

    let asset_id_uuid = match Uuid::from_slice(&transaction_to_delete.asset_id) {
        Ok(uuid) => {
            log::debug!("Parsed asset_id to UUID: {}", uuid);
            uuid
        }
        Err(e) => {
            log::error!("Invalid asset UUID: {}", e);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed(format!(
                "Invalid asset UUID: {}",
                e
            )));
        }
    };

    // Delete the payment record
    log::debug!("Deleting transaction with transaction_id: {}", transaction_id);
    let delete_result = match transaction::Entity::delete_by_id(transaction_id.as_bytes().to_vec())
        .exec(&txn)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            log::error!("Failed to delete transaction: {}", err);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::DatabaseError(err.to_string()));
        }
    };

    log::debug!("Delete result: rows_affected = {}", delete_result.rows_affected);

    if delete_result.rows_affected == 0 {
        log::error!("No rows affected during delete operation for transaction_id: {}", transaction_id);
        txn.rollback().await.ok(); // Rollback on error
        return Err(RepositoryError::NotFound(format!(
            "Payment record {} not found for user {} during delete operation",
            transaction_id, user_id
        )));
    }

    // Update the balance in the CurrentSheet table
    log::debug!("Fetching current sheet for asset_id: {}", asset_id_uuid);
    let current_sheet = match balance_repo
        .get_current_sheet_by_asset_id(user_id, asset_id_uuid)
        .await
    {
        Ok(Some(sheet)) => sheet,
        Ok(None) => {
            log::error!("Current sheet not found for asset_id: {}", asset_id_uuid);
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

    log::debug!("Current sheet found: {:?}", current_sheet);

    let new_balance = match Decimal::from_f64(amount_to_add_back) {
        Some(amount) => {
            log::debug!("Calculating new balance: current_balance = {}, amount_to_add_back = {}", current_sheet.balance, amount);
            current_sheet.balance + amount
        }
        None => {
            log::error!("Failed to convert amount_to_add_back to Decimal: {}", amount_to_add_back);
            txn.rollback().await.ok(); // Rollback on error
            return Err(RepositoryError::OperationFailed(
                "Failed to convert amount_to_add_back to Decimal".to_string(),
            ));
        }
    };

    log::debug!("New balance calculated: {}", new_balance);

    if let Err(err) = balance_repo
        .update_current_sheet(user_id, asset_id_uuid, Some(new_balance.to_f64().unwrap()))
        .await
    {
        log::error!("Failed to update balance in CurrentSheet table: {}", err);
        txn.rollback().await.ok(); // Rollback on error
        return Err(err);
    }

    log::debug!("Balance updated successfully in CurrentSheet table.");

    // Commit the transaction
    log::debug!("Committing transaction...");
    txn.commit().await.map_err(|err| {
        log::error!("Failed to commit transaction: {}", err);
        RepositoryError::DatabaseError(format!("Failed to commit transaction: {}", err))
    })?;

    log::info!("Payment record deleted successfully for transaction_id: {}", transaction_id);

    Ok(())
}


    async fn get_payment_record_by_id(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    ) 
        -> Result<Option<transaction::Model>, RepositoryError>
    {
        // Query the database to find the payment record by ID and ensure it belongs to the user
        let payment_record = transaction::Entity::find()
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the payment record if found, or None if not found
        Ok(payment_record)
    }


    async fn get_all_payment_record_by_user(
        &self, 
        user_id: Uuid
    ) 
        -> Result<Vec<transaction::Model>, RepositoryError>
    {

        let payment_model = transaction_type::Entity::find()
            .filter(transaction_type::Column::Name.eq("payment"))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        let payment_uuid = match payment_model {
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

        // Query the database to retrieve all payment records for the given user
        let payment_records = transaction::Entity::find()
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .filter(transaction::Column::TransactionTypeId.eq(payment_uuid.as_bytes().to_vec())) // Filter by transaction type
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of payment records
        Ok(payment_records)
    }


}