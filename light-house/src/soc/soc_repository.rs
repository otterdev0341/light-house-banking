use thiserror::Error;

// Use RepositoryError::NotFound when the resource does not exist.
// Use RepositoryError::DatabaseError for general database errors.
// Use RepositoryError::UniqueConstraintViolation for unique constraint violations.
// Use RepositoryError::ForeignKeyConstraintViolation for foreign key violations.
// Use RepositoryError::InvalidInput for invalid input data.
// Use RepositoryError::OperationFailed for generic operation failures.




#[derive(Error, Debug)]
pub enum RepositoryError {
    //When to Use: The requested resource (e.g., a user) does not exist in the database.
    #[error("Resource not found: {0}")]
    NotFound(String),

    //When to Use: A general database error occurs (e.g., connection issues, query syntax errors).
    #[error("Database error: {0}")]
    DatabaseError(String), // Can wrap a specific DB error string or type


    // When to Use: A unique constraint is violated (e.g., trying to insert a user with an email that already exists).
    #[error("Unique constraint violation: {0}")]
    UniqueConstraintViolation(String),


    // When to Use: A foreign key constraint is violated (e.g., trying to delete a user referenced in another table).
    #[error("Foreign key constraint violation: {0}")]
    ForeignKeyConstraintViolation(String),


    // When to Use: The input provided for the operation is invalid (e.g., invalid email format, missing required fields).
    #[error("Invalid input for repository operation: {0}")]
    InvalidInput(String),


    // When to Use: A generic error for when an operation fails but does not fit into the other categories.
    #[error("Operation failed: {0}")]
    OperationFailed(String),
    // Add other specific data access errors as needed

    #[error("Permission denied: {0}")]
    PermissionDenied(String), // For permission-related errors
}

// Example: If you are using SeaORM, you might convert its DbErr
impl From<sea_orm::DbErr> for RepositoryError {
    fn from(err: sea_orm::DbErr) -> Self {
        RepositoryError::DatabaseError(err.to_string()) // Basic conversion
    }
}


