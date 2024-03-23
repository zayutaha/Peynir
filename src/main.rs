pub mod cli;
pub mod models;
pub mod token;

use std::{
    fs::OpenOptions,
    io::{BufRead, BufWriter},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use cli::Opt;
use eyre::{ContextCompat, Result};
use models::Token;
use token::{delete_token, display_tokens, load_tokens};

use crate::token::generate_token;

fn main() -> Result<()> {
    let args = Opt::parse();
    let mut accounts = load_tokens("tokens.json".to_string()).unwrap();
    match args {
        Opt::Get { name } => {
            let a = accounts
                .iter()
                .find(|x| x.name == name)
                .wrap_err("Account not found")
                .unwrap();
            let token = generate_token(a.clone())?;
            println!("{:?}", token);
        }
        Opt::Add { name, secret, time } => {
            let new_account = Token { name, secret, time };
            accounts.push(new_account);
            println!("Account added!");
        }
        Opt::Delete { name } => {
            delete_token(&mut accounts, name)?;
        }
        Opt::List {} => loop {
            let mut tokens = Vec::new();
            for a in accounts.clone() {
                tokens.push((a.name.clone(), generate_token(a.clone()).unwrap()));
            }
            display_tokens(tokens)?;
            let mut tokens = Vec::new();
            for a in accounts.clone() {
                tokens.push((a.name.clone(), generate_token(a.clone()).unwrap()));
            }
            std::thread::sleep(Duration::from_secs(
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() % 30,
            ));
        },
    }
    save_accounts(accounts, String::from("./tokens.json"))?;
    Ok(())
}

fn save_accounts(accounts: Vec<Token>, path: String) -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .read(false)
        .write(true)
        .truncate(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &accounts)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use eyre::Result;
    use tempdir::TempDir;

    use crate::{models::Token, save_accounts, token::load_tokens};

    fn init_dir() -> Result<(TempDir, String)> {
        let tmp_dir = TempDir::new("test_dir")?;
        let file_path = tmp_dir.path().join("tokens.json");
        let path = file_path.to_str().unwrap().to_string();
        Ok((tmp_dir, path))
    }

    #[test]
    fn user_creates_account() -> Result<()> {
        let account = Token {
            name: "John".to_string(),
            secret: "xyz".to_string(),
            time: 30,
        };
        let (tmp_dir, path) = init_dir()?;
        save_accounts(vec![account.clone()], path.clone())?;
        assert_eq!(load_tokens(path)?, vec![account]);
        tmp_dir.close()?;
        Ok(())
    }
}
