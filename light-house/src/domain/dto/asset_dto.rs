use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;





#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateAssetDto{
    #[validate(length(min = 1, message = "The name must not be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "The asset type id must not be empty"))]
    pub asset_type_id: String,
}



#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateAssetDto {
    pub name: Option<String>,
    pub asset_type_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryUpdateAssetDto{
    pub id: i32,
    pub name: String,
    pub asset_type_id: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListAssetDto {
    pub length: i32,
    pub data: Vec<ResEntryUpdateAssetDto>,
}