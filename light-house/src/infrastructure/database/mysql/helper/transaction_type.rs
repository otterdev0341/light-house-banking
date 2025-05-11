use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum TransactionType {
    Income,
    Payment,
    Transfer,
}