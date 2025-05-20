

use std::sync::Arc;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::{entities::transaction_type, req_repository::transaction_repository::TransactionTypeRepositoryUtility}, soc::soc_repository::RepositoryError};









pub struct TransactionTypeRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>,
}
impl TransactionTypeRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        TransactionTypeRepositoryImpl { db_pool }
    }
}



#[async_trait::async_trait]
impl TransactionTypeRepositoryUtility for TransactionTypeRepositoryImpl{

    async fn get_transaction_type_by_id(&self, _user_id: Uuid, transaction_type_id: Uuid) -> Result<Option<transaction_type::Model>, RepositoryError>
    {
        let transaction_type = transaction_type::Entity::find()
            .filter(transaction_type::Column::Id.eq(transaction_type_id))
            .one(self.db_pool.as_ref())
            .await
            .map_err(|_| RepositoryError::DatabaseError("Failed to fetch transaction type".to_string()))?;

        Ok(transaction_type)
    }
    async fn get_all_transaction_types_by_user(&self) -> Result<Vec<transaction_type::Model>, RepositoryError>
    {
        let transaction_types = transaction_type::Entity::find()
            .all(self.db_pool.as_ref())
            .await
            .map_err(|_| RepositoryError::DatabaseError("Failed to fetch transaction types".to_string()))?;

        Ok(transaction_types)
    }
}