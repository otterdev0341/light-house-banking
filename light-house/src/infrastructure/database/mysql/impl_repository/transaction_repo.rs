use std::sync::Arc;

use sea_orm::DatabaseConnection;





pub struct TransactionRepositoryImpl{
    pub transaction_repo: Arc<DatabaseConnection>
}

impl TransactionRepositoryImpl{
    pub fn new(transaction_repo: Arc<DatabaseConnection>) -> Self {
        TransactionRepositoryImpl {
            transaction_repo
        }
    }
}