use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::cmp::Ordering;



#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, ToSchema)]
#[schema(value_type = f64, format = "decimal")]
pub struct DecimalWrapper(pub Decimal);

impl PartialOrd for DecimalWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}


