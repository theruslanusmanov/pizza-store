use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct ProductsDto {
    pub id: i64,
    pub name: String
}