use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    str::FromStr,
};

use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use totp_rs::{Algorithm, Secret, TOTP};

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

impl From<EncryptionAlgo> for Algorithm {
    fn from(value: EncryptionAlgo) -> Self {
        match value {
            EncryptionAlgo::SHA1 => Algorithm::SHA1,
            EncryptionAlgo::SHA256 => Algorithm::SHA256,
            EncryptionAlgo::SHA512 => Algorithm::SHA512,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Account {
    pub account_name: String,
    pub secret: String,
    pub time: u64,
    pub algorithm: EncryptionAlgo,
    pub digits: Option<usize>,
    pub skew: Option<u8>,
}

impl Account {
    pub fn load_tokens(path: String) -> Result<Vec<Account>> {
        let file = std::fs::read_to_string(path)?;
        let account: Vec<Account> = serde_json::from_str(&file)?;
        return Ok(account);
    }
}

pub fn generate_token(account: Account) -> Result<String> {
    let secret = Secret::Encoded(account.secret).to_string();
    let totp = TOTP::new(
        account.algorithm.into(),
        account.digits.unwrap_or(6),
        account.skew.unwrap_or(1),
        account.time,
        secret.into_bytes(),
    )
    .unwrap();
    let token = totp.generate_current()?;
    Ok(token)
}

pub fn add_token(accounts: &mut Vec<Account>, account: Account) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./token.json")?;
    accounts.push(account);
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &accounts)?;
    Ok(())
}
