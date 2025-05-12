use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;




#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateExpenseDto {
    #[validate(length(min = 1, message = "The name must not be empty"))]
    pub description: String,
    pub expense_type_id: i32,

}



#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateExpenseDto {
    pub description: Option<String>,
    pub expense_type_id: Option<i32>
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryExpenseDto {
    pub id: i32,
    pub description: String,
    pub expense_type_id: i32,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListExpenseDto {
    pub length: i32,
    pub data: Vec<ResEntryExpenseDto>,
}