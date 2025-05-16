use std::sync::Arc;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{entities::{transaction, transaction_type}, req_repository::transaction_repository::TransactionTypeRepositoryUtility}, soc::soc_repository::RepositoryError};





pub struct TransactionRepoImpl {
    db_pool: Arc<DatabaseConnection>,
}

impl TransactionRepoImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl TransactionTypeRepositoryUtility for TransactionRepoImpl
{
    async fn get_transaction_type_by_id(
        &self, 
        user_id: Uuid, 
        transaction_type_id: Uuid
    ) 
        -> Result<Option<transaction_type::Model>, RepositoryError>
    { 
        match transaction_type::Entity::find()
            .filter(transaction::Column::UserId.eq(user_id))
            .filter(transaction::Column::Id.eq(transaction_type_id))
            .one(self.db_pool.as_ref())
            .await
        {
            Ok(Some(transaction)) => Ok(Some(transaction)),
            Ok(None) => Ok(None),
            Err(err) => Err(RepositoryError::DatabaseError(err.to_string())),
        }
    }
    
    async fn get_all_transaction_types_by_user(
        &self, 
    ) 
        -> Result<Vec<transaction_type::Model>, RepositoryError>
    {
        match transaction_type::Entity::find()
            .all(self.db_pool.as_ref())
            .await
        {
            Ok(transactions) => Ok(transactions),
            Err(err) => Err(RepositoryError::DatabaseError(err.to_string())),
        }
    }
}