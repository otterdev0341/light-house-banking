use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;



#[derive(Debug, Serialize,Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqTransactionTypeDto {
    #[validate(length(min = 1, message = "The name must not be empty"))]
    pub name: String,
}


#[derive(Debug, Serialize,Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateTransactionTypeDto {
    pub name: Option<String>,
}


#[derive(Debug, Serialize,Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryTransactionTypeDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Debug, Serialize,Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListTransactionTypeDto {
    pub length: i32,
    pub data: Vec<ResEntryTransactionTypeDto>,
}