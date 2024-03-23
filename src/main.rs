pub mod cli;
pub mod list;
pub mod models;
pub mod token;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::token::{add_token, generate_token};
use clap::Parser;
use cli::Opt;
use eyre::{ContextCompat, Result};
use list::display_tokens;
use models::Token;
use token::{delete_token, load_tokens};

fn main() -> Result<()> {
    let args = Opt::parse();
    let mut accounts = load_tokens("tokens.json".to_string()).unwrap();
    match args {
        Opt::Get { account } => {
            let a = accounts
                .iter()
                .find(|x| x.account_name == account)
                .wrap_err("Account not found")
                .unwrap();
            let token = generate_token(a.clone())?;
            println!("{:?}", token);
        }
        Opt::Add { name, secret, time } => {
            let new_account = Token {
                account_name: name,
                secret,
                time,
            };
            add_token(&mut accounts, new_account, None)?;
            println!("Account added!");
        }
        Opt::Delete { name } => {
            delete_token(&mut accounts, name, None)?;
        }
        Opt::List {} => loop {
            let mut tokens = Vec::new();
            for a in accounts.clone() {
                tokens.push((a.account_name.clone(), generate_token(a.clone()).unwrap()));
            }
            display_tokens(tokens)?;
            let mut tokens = Vec::new();
            for a in accounts.clone() {
                tokens.push((a.account_name.clone(), generate_token(a.clone()).unwrap()));
            }
            std::thread::sleep(Duration::from_secs(
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() % 30,
            ));
        },
    }
    // let path = std::env::args().nth(1).unwrap();
    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use eyre::Result;
    use tempdir::TempDir;

    use crate::{
        models::Token,
        token::{add_token, delete_token, load_tokens},
    };

    fn init_account() -> Result<(TempDir, String)> {
        let test_account = Token {
            account_name: "Haduba".into(),
            secret: "xyz".into(),
            time: 30,
        };
        let tmp_dir = TempDir::new("test_dir")?;
        let file_path = tmp_dir.path().join("tokens.json");
        let path = file_path.to_str().unwrap().to_string();
        File::create(path.clone())?;
        println!("{:?}", path.clone());
        add_token(&mut Vec::new(), test_account.clone(), Some(path.clone()))?;
        Ok((tmp_dir, path))
    }

    #[test]
    fn user_adds_account() -> Result<()> {
        let test_account = Token {
            account_name: "Haduba".into(),
            secret: "xyz".into(),
            time: 30,
        };
        let (dir, path) = init_account()?;
        let loaded_accounts = load_tokens(path.clone())?;
        dir.close()?;
        println!("{:?}", loaded_accounts);
        assert_eq!(loaded_accounts, vec![test_account]);
        Ok(())
    }

    #[test]
    fn user_deletes_account() -> Result<()> {
        let (dir, path) = init_account()?;
        let mut tokens = load_tokens(path.clone())?;
        delete_token(&mut tokens, "Haduba".to_string(), Some(path.clone()))?;
        let loaded_accounts = load_tokens(path.clone())?;
        dir.close()?;
        assert_eq!(loaded_accounts, vec![]);
        Ok(())
    }
}
