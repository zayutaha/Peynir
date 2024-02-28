use eyre::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum EncryptionAlgo {
    SHA1,
    SHA256,
    SHA512,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub token: String,
    pub algorithm: EncryptionAlgo,
}

impl Token {
    pub fn load_token(path: String) -> Result<Token> {
        let file = std::fs::read_to_string(path)?;
        let token: Token = serde_json::from_str(&file)?;
        return Ok(token);
    }
}
