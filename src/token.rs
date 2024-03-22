use eyre::Result;
use std::{fs::OpenOptions, io::BufWriter};

use crate::models::Account;
pub fn generate_token(account: Account) -> Result<String> {
    let code = otp::make_totp(&account.secret, account.time, 0)?;
    Ok(code.to_string())
}

pub fn add_token(
    accounts: &mut Vec<Account>,
    account: Account,
    path: Option<String>,
) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.unwrap_or("./token.json".into()))?;
    accounts.push(account);
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &accounts)?;
    Ok(())
}

pub fn delete_token(
    accounts: &mut Vec<Account>,
    account: String,
    path: Option<String>,
) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(path.unwrap_or("./token.json".into()))?;
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
