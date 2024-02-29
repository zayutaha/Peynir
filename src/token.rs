use eyre::Result;
use serde::Deserialize;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Debug, Deserialize)]
pub enum EncryptionAlgo {
    SHA1,
    SHA256,
    SHA512,
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

#[derive(Debug, Deserialize)]
pub struct Account {
    pub secret: String,
    pub time: u64,
    pub algorithm: EncryptionAlgo,
    pub digits: Option<usize>,
    pub skew: Option<u8>,
}

impl Account {
    pub fn load_token(path: String) -> Result<Account> {
        let file = std::fs::read_to_string(path)?;
        let account: Account = serde_json::from_str(&file)?;
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
