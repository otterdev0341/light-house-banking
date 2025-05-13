
use serde::Serialize;
use thiserror::Error;

use super::soc_repository::RepositoryError;

#[derive(Serialize, Debug, Clone)] // Serialize for potential inclusion in API error details
pub struct ValidationErrorDetail {
    pub field: String,
    pub message: String,
}

#[derive(Error, Debug)]
pub enum UsecaseError {
    #[error("Validation failed")]
    ValidationFailed(Vec<ValidationErrorDetail>),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Operation conflict: {0}")]
    Conflict(String), // e.g., trying to delete an item that's in use

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),

    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError), // Allows easy conversion from RepositoryError
                                              // Add other business-specific errors
}

impl UsecaseError {
    pub fn new_validation_error(field: &str, message: &str) -> Self {
        UsecaseError::ValidationFailed(vec![ValidationErrorDetail { field: field.to_string(), message: message.to_string() }])
    }
}