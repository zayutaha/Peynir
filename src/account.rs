use eyre::Result;
use std::fs::File;

use crate::models::Account;
pub fn generate(account: &Account) -> Result<String> {
    let code = otp::make_totp(&account.secret, account.time, 0)?;
    Ok(code.to_string())
}

pub fn load(path: String) -> Result<Vec<Account>> {
    let file = std::fs::read_to_string(path).unwrap_or_else(|_| {
        File::create("accounts.json").unwrap();
        String::new()
    });
    let account: Vec<Account> = serde_json::from_str(&file).unwrap_or(Vec::new());
    Ok(account)
}

pub fn display(accounts: &Vec<(String, String)>) -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    println!("┌─────────────────────┬─────────┐");
    println!("|         Name        |  Code   |");
    println!("├─────────────────────┼─────────┤");
    for account in accounts {
        println!("|{:<20} | {:<8}|", account.0, account.1);
    }
    println!("└─────────────────────┴─────────┘");
    Ok(())
}
