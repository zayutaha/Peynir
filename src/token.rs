use eyre::Result;
use std::{
    fs::{File, OpenOptions},
    io::BufWriter,
};

use crate::models::Token;
pub fn generate_token(account: Token) -> Result<String> {
    let code = otp::make_totp(&account.secret, account.time, 0)?;
    Ok(code.to_string())
}

pub fn add_token(accounts: &mut Vec<Token>, account: Token, path: Option<String>) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.unwrap_or("./tokens.json".into()))?;
    accounts.push(account);
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &accounts)?;
    Ok(())
}

pub fn delete_token(
    accounts: &mut Vec<Token>,
    account: String,
    path: Option<String>,
) -> Result<()> {
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .open(path.unwrap_or("./tokens.json".into()))?;
    let index = accounts
        .iter()
        .position(|x| x.account_name == account)
        .expect("Token should exist");
    accounts.remove(index);
    println!("{:?} removed!", account);
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, accounts)?;
    Ok(())
}

pub fn load_tokens(path: String) -> Result<Vec<Token>> {
    let file = std::fs::read_to_string(path).unwrap_or_else(|_| {
        File::create("tokens.json".to_string()).unwrap();
        "".to_string()
    });
    let account: Vec<Token> = serde_json::from_str(&file).unwrap_or(Vec::new());
    Ok(account)
}
