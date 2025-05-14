use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{dto::transaction_dto::{ReqCreatePaymentDto, ReqUpdatePaymentDto}, entities::transaction, req_repository::transaction_repository::RecordPaymentRepositoryUtility}, soc::soc_repository::RepositoryError};





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
        // Create the ActiveModel for the payment record
        let new_payment_record = transaction::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()), // Generate a new UUID for the transaction
            transaction_type_id: Set(payment_record_dto.transaction_type_id.as_bytes().to_vec()),
            amount: Set(payment_record_dto.amount),
            expense_id: Set(Some(payment_record_dto.expense_id.as_bytes().to_vec())),
            contact_id: Set(Some(payment_record_dto.contact_id.as_bytes().to_vec())),
            note: Set(payment_record_dto.note),
            user_id: Set(user_id.as_bytes().to_vec()),
            ..Default::default()
        };

        // Insert the payment record into the database
        let inserted_payment_record = new_payment_record
            .insert(self.db_pool.as_ref())
            .await
            .map_err(|err| {
                if let sea_orm::DbErr::Exec(exec_err) = &err {
                    if exec_err.to_string().contains("FOREIGN KEY") {
                        return RepositoryError::OperationFailed(
                            "Invalid foreign key reference in the payment record".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(inserted_payment_record)
    }


    async fn update_payment_record(
        &self, 
        user_id: Uuid, 
        transaction_id: Uuid, 
        payment_record_dto: ReqUpdatePaymentDto
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

        // Update fields if they are provided in the DTO
        if let Some(transaction_type_id) = payment_record_dto.transaction_type_id {
            active_model.transaction_type_id = Set(transaction_type_id.as_bytes().to_vec());
        }
        if let Some(amount) = payment_record_dto.amount {
            active_model.amount = Set(amount);
        }
        if let Some(expense_id) = payment_record_dto.expense_id {
            active_model.expense_id = Set(Some(expense_id.as_bytes().to_vec()));
        }
        if let Some(contact_id) = payment_record_dto.contact_id {
            active_model.contact_id = Set(Some(contact_id.as_bytes().to_vec()));
        }
        if let Some(note) = payment_record_dto.note {
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
                            "Invalid foreign key reference in the payment record".to_string(),
                        );
                    }
                }
                RepositoryError::DatabaseError(err.to_string())
            })?;

        Ok(updated_transaction)
    }


    async fn delete_payment_record(
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
                "Payment record with ID {} not found for user {}",
                transaction_id, user_id
            )));
        }

        // Delete the payment record
        transaction::Entity::delete_many()
            .filter(transaction::Column::Id.eq(transaction_id.as_bytes().to_vec())) // Filter by transaction ID
            .exec(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

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
        // Query the database to retrieve all payment records for the given user
        let payment_records = transaction::Entity::find()
            .filter(transaction::Column::UserId.eq(user_id.as_bytes().to_vec())) // Filter by user ID
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        // Return the list of payment records
        Ok(payment_records)
    }
}