use serde::{Deserialize, Serialize};
use utoipa::ToSchema;




#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResGenderDto{
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListGenderDto{
    pub length: i32,
    pub data: Vec<ResGenderDto>,
}