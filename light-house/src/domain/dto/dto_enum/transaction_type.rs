use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub enum TransactionTypeVariant {
    Income,
    Payment,
    Transfer,
}