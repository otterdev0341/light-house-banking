use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateAssetTypeDto {
    #[validate(length(min = 1, message = "The name must not be empty"))]
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateAssestTypeDto {
    
    pub name: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryAssetTypeDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListAssestTypeDto {
    pub length: i32,
    pub data: Vec<ResEntryAssetTypeDto>,
}