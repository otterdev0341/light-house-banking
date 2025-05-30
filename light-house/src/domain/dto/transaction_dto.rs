use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;


// >>>>>>>> Payment <<<<<<<<
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreatePaymentDto {
    #[validate(length(min = 1, message = "The transaction_type_id must not be empty"))]
    pub transaction_type_id: String,
    #[validate(range(min = 0.1, message = "The amount must not be empty"))]
    pub amount: f64,
    #[validate(length(min = 1, message = "The expense_id must not be empty"))]
    pub expense_id: String,
    #[validate(length(min = 1, message = "The asset_id must not be empty"))]
    pub asset_id: String,
    #[validate(length(min = 1, message = "The contact_id must not be empty"))]
    pub contact_id: String,
    #[validate(length(min = 1, message = "The note must not be empty"))]
    pub note: String,
    #[validate(length(min = 1, message = "The created_at must not be empty"))]
    pub created_at: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdatePaymentDto {
    //pub transaction_type_id: Option<String>,
    pub amount: Option<f64>,
    pub expense_id: Option<String>,
    pub asset_id: Option<String>,
    pub contact_id: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryPaymentDto{
    pub id: String,
    pub transaction_type_name: String,
    pub amount: f64,
    pub expense_name: String,
    pub contact_name: String,
    pub asset_name: String,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListPaymentDto{
    pub length: i32,
    pub data: Vec<ResEntryPaymentDto>,
}

// >>>>>>>> Income <<<<<<<<
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateIncomeDto {
    #[validate(length(min = 1, message = "The transaction_type_id must not be empty"))]
    pub transaction_type_id: String,
    pub amount: f64,
    #[validate(length(min = 1, message = "The aseet_id must not be empty"))]
    pub asset_id: String,
    #[validate(length(min = 1, message = "The contact_id must not be empty"))]
    pub contact_id: String,
    #[validate(length(min = 1, message = "The note must not be empty"))]
    pub note: String,
    #[validate(length(min = 1, message = "The created_at must not be empty"))]
    pub created_at: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateIncomeDto {
    pub amount: Option<f64>,
    pub asset_id: Option<String>,
    pub contact_id: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryIncomeDto{
    pub id: String,
    pub transaction_type_name: String,
    pub amount: f64,
    pub asset_name: String,
    pub contact_name: String,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListIncomeDto{
    pub length: i32,
    pub data: Vec<ResEntryIncomeDto>,
}

// >>>>>>>> Transfer <<<<<<<<
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateTransferDto {
    #[validate(length(min = 1, message = "The transaction_type_id must not be empty"))]
    pub transaction_type_id: String,
    #[validate(range(min = 0.1, message = "The amount must not be empty"))]
    pub amount: f64,
    #[validate(length(min = 1, message = "The asset_id must not be empty"))]
    pub asset_id: String,
    #[validate(length(min = 1, message = "The destination_asset_id must not be empty"))]
    pub destination_asset_id: String,
    #[validate(length(min = 1, message = "The contact_id must not be empty"))]
    pub contact_id: String,
    #[validate(length(min = 1, message = "The note must not be empty"))]
    pub note: String,
    #[validate(length(min = 1, message = "The created_at must not be empty"))]
    pub created_at: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateTransferDto {
    pub amount: Option<f64>,
    pub asset_id: Option<String>,
    pub destination_asset_id: Option<String>,
    pub contact_id: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryTransferDto{
    pub id: String,
    pub transaction_type_name: String,
    pub amount: f64,
    pub asset_name: String,
    pub destination_asset_name: String,
    pub contact_name: String,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListTransferDto{
    pub length: i64,
    pub data: Vec<ResEntryTransferDto>,
}