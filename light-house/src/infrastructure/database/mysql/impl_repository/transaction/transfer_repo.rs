use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{dto::transaction_dto::{ReqCreateTransferDto, ReqUpdateTransferDto}, entities::transaction, req_repository::transaction_repository::TranferRepositoryUtility}, soc::soc_repository::RepositoryError};





pub struct TransferRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}

impl TransferRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
    
}

#[async_trait::async_trait]
impl TranferRepositoryUtility for TransferRepositoryImpl {

    
    async fn create_transfer(
        &self, 
        user_id: Uuid, 
        transfer_dto: ReqCreateTransferDto
    ) 
        -> Result<transaction::Model, RepositoryError> 
    {
        // Create the ActiveModel for the transfer transaction
        let new_transfer = transaction::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the transaction
            transaction_type_id: Set(transfer_dto.transaction_type_id.as_bytes().to_vec()),
            amount: Set(transfer_dto.amount),
            asset_id: Set(transfer_dto.asset_id.as_bytes().to_vec()),
            destination_asset_id: Set(Some(transfer_dto.destination_asset_id.as_bytes().to_vec())),
            contact_id: Set(Some(transfer_dto.contact_id.as_bytes().to_vec())),
            note: Set(transfer_dto.note),
            user_id: Set(user_id.as_bytes().to_vec()),
            ..Default::default()
        };

        // Insert the transfer transaction into the database
        let inserted_transfer = new_transfer
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("FOREIGN KEY") {
                        return RepositoryError::OperationFailed(
                            "Invalid foreign key reference in the transfer".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(inserted_transfer)
    }

    
    async fn update_transfer(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid, 
        transfer_dto: ReqUpdateTransferDto
    ) 
        -> Result<transaction::Model, RepositoryError> 
    {
        // Ensure the transaction belongs to the user
        let transaction_exists = transaction::Entity::find()
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if transaction_exists.is_none() {
            return Err(RepositoryError::NotFound(format!(
                "Transaction with ID {} not found for user {}",
                transaction_id, user_id
            )));
        }

        // Convert the existing transaction into an ActiveModel for updating
        let mut active_model: transaction::ActiveModel = transaction_exists.unwrap().into();

        if let Some(amount) = transfer_dto.amount {
            active_model.amount = Set(amount);
        }
        if let Some(asset_id) = transfer_dto.aseet_id {
            active_model.asset_id = Set(asset_id.as_bytes().to_vec());
        }
        if let Some(destination_asset_id) = transfer_dto.destination_asset_id {
            active_model.destination_asset_id = Set(Some(destination_asset_id.as_bytes().to_vec()));
        }
        if let Some(contact_id) = transfer_dto.contact_id {
            active_model.contact_id = Set(Some(contact_id.as_bytes().to_vec()));
        }
        if let Some(note) = transfer_dto.note {
            active_model.note = Set(note);
        }


        // Save the updated transaction to the database
        let updated_transaction = active_model
            .update(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("FOREIGN KEY") {
                        return RepositoryError::OperationFailed(
                            "Invalid foreign key reference in the transfer".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(updated_transaction)
    }

    
    async fn delete_transfer(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    ) 
        -> Result<(), RepositoryError> 
    {
        // Ensure the transaction belongs to the user
        let transaction_exists = transaction::Entity::find()
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Ensure it belongs to the user
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        if transaction_exists.is_none() {
            return Err(RepositoryError::NotFound(format!(
                "Transaction with ID {} not found for user {}",
                transaction_id, user_id
            )));
        }

        // Delete the transaction
        transaction::Entity::delete_many()
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(())
    }

    
    async fn get_transfer_by_id(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid
    ) 
        -> Result<Option<transaction::Model>, RepositoryError> 
    {
        // Query the database to find the transaction by ID and ensure it belongs to the user
        let transaction = transaction::Entity::find()
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
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
        // Query the database to retrieve all transactions for the given user
        let transactions = transaction::Entity::find()
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of transactions
        Ok(transactions)
    }
}