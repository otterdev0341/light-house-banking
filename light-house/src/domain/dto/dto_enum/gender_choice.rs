use serde::{Deserialize, Serialize};
use utoipa::ToSchema;




#[derive(Deserialize, Serialize, ToSchema)]
pub enum GenderVariant {
    Male,
    Female,
    Other
}