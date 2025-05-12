use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;



#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateTransactionDto {
    #[validate(length(min = 1, message = "The transaction type id must not be empty"))]
    pub transaction_type_id: String,
    #[validate(range(min = 0.01, message = "The amount must be greater than 0"))]
    pub amount: f32,
    #[validate(length(min = 1, message = "The asset id must not be empty"))]
    pub aseet_id: String,
    pub destination_asset_id: Option<String>,
    pub expense_id: Option<String>,
    pub contact_id: Option<String>,
    #[validate(length(min = 1, message = "The note must not be empty"))]
    pub note: String,
}


#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateTransactionDto {
    pub transaction_type_id: Option<String>,
    pub amount: Option<f32>,
    pub aseet_id: Option<String>,
    pub destination_asset_id: Option<String>,
    pub expense_id: Option<String>,
    pub contact_id: Option<String>,
    pub note: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryTransactionDto {
    pub id: String,
    pub transaction_type_id: String,
    pub amount: f32,
    pub aseet_id: String,
    pub destination_asset_id: Option<String>,
    pub expense_id: Option<String>,
    pub contact_id: Option<String>,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}



#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListTransactionDto {
    pub length: i32,
    pub data: Vec<ResEntryTransactionDto>,
}