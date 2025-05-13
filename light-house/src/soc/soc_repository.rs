use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String), // Can wrap a specific DB error string or type

    #[error("Unique constraint violation: {0}")]
    UniqueConstraintViolation(String),

    #[error("Foreign key constraint violation: {0}")]
    ForeignKeyConstraintViolation(String),

    #[error("Invalid input for repository operation: {0}")]
    InvalidInput(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
    // Add other specific data access errors as needed
}

// Example: If you are using SeaORM, you might convert its DbErr
impl From<sea_orm::DbErr> for RepositoryError {
    fn from(err: sea_orm::DbErr) -> Self {
        RepositoryError::DatabaseError(err.to_string()) // Basic conversion
    }
}