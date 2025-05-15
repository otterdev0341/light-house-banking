use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;



#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateContactDto {
    #[validate(length(min = 1, message = "The name must not be empty"))]
    pub name: String,
    pub business_name: String,
    pub phone: String,
    pub description: String,
    #[validate(length(min = 1, message = "The contact type id must not be empty"))]
    pub contact_type_id: String,
}


#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateContactDto {
    pub name: Option<String>,
    pub business_name: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub contact_type_id: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryContactDto {
    pub id: String,
    pub name: String,
    pub business_name: String,
    pub phone: String,
    pub description: String,
    pub contact_type_name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListContactDto {
    pub length: i32,
    pub data: Vec<ResEntryContactDto>,
}