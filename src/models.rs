use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Account {
    pub name: String,
    pub secret: String,
    pub time: u64,
}
