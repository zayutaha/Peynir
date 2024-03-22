use eyre::eyre;
use std::str::FromStr;

use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum EncryptionAlgo {
    SHA1,
    SHA256,
    SHA512,
}

impl FromStr for EncryptionAlgo {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SHA1" => Ok(EncryptionAlgo::SHA1),
            "SHA256" => Ok(EncryptionAlgo::SHA256),
            "SHA512" => Ok(EncryptionAlgo::SHA512),
            _ => Err(eyre!("Invalid Algorithm")),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Token {
    pub account_name: String,
    pub secret: String,
    pub time: u64,
    pub algorithm: EncryptionAlgo,
    pub digits: Option<usize>,
    pub skew: Option<u8>,
}

impl Token {
    pub fn load_tokens(path: String) -> Result<Vec<Token>> {
        let file = std::fs::read_to_string(path)?;
        let account: Vec<Token> = serde_json::from_str(&file).unwrap_or(Vec::new());
        Ok(account)
    }
}
