use serde::{Deserialize, Serialize};
use utoipa::ToSchema;





#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResCurrentSheetDto {
    pub id: String,
    pub asset_name: String,
    pub balance: f64,
    pub last_transaction_id: Option<String>,
    pub updated_at: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResListCurrentSheetDto {
    pub length: i32,
    pub data: Vec<ResCurrentSheetDto>,
}

