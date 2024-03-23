use eyre::Result;
use std::fs::File;

use crate::models::Token;
pub fn generate_token(account: Token) -> Result<String> {
    let code = otp::make_totp(&account.secret, account.time, 0)?;
    Ok(code.to_string())
}

pub fn delete_token(accounts: &mut Vec<Token>, account: String) -> Result<()> {
    let index = accounts
        .iter()
        .position(|x| x.name == account)
        .expect("Token should exist");
    accounts.remove(index);
    println!("{:?} removed!", account);
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

pub fn display_tokens(tokens: Vec<(String, String)>) -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    println!("┌──────────┬──────────────┐");
    println!("    Name         Code ");
    println!("├──────────┼──────────────┤");
    for token in tokens.iter() {
        println!("|{:<9} | {:<13}|", token.0, token.1);
    }
    println!("└──────────┴──────────────┘");
    Ok(())
}
