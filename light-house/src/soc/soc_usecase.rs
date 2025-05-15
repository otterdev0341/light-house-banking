use serde::Serialize;
use thiserror::Error;

use super::soc_repository::RepositoryError;

#[derive(Error, Debug, Serialize)]
pub enum UsecaseError {
    #[error("Validation failed")]
    ValidationFailed(Vec<ValidationErrorDetail>),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Operation conflict: {0}")]
    Conflict(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}

// Validation error detail structure
#[derive(Debug, Serialize)]
pub struct ValidationErrorDetail {
    pub field: String,
    pub message: String,
}

// Implement From<RepositoryError> for UsecaseError
impl From<RepositoryError> for UsecaseError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound(msg) => UsecaseError::ResourceNotFound(msg),
            RepositoryError::DatabaseError(msg) => UsecaseError::Unexpected(msg),
            RepositoryError::UniqueConstraintViolation(msg) => UsecaseError::Conflict(msg),
            RepositoryError::ForeignKeyConstraintViolation(msg) => UsecaseError::Conflict(msg),
            RepositoryError::InvalidInput(msg) => UsecaseError::ValidationFailed(vec![
                ValidationErrorDetail {
                    field: "input".to_string(),
                    message: msg,
                },
            ]),
            RepositoryError::OperationFailed(msg) => UsecaseError::Unexpected(msg),
            RepositoryError::PermissionDenied(msg) => UsecaseError::PermissionDenied(msg),
        }
    }
}

impl UsecaseError {
    pub fn new_validation_error(field: &str, message: &str) -> Self {
        UsecaseError::ValidationFailed(vec![ValidationErrorDetail {
            field: field.to_string(),
            message: message.to_string(),
        }])
    }
}