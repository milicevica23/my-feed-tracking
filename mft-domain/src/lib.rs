pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Feeling {
    pub id: i32,
    pub name: String,
}

pub struct FeelingTransaction {
    pub id: i32,
    pub timestamp: String,
    pub value: i32,
}
