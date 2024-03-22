use eyre::{ContextCompat, Result};
use models::{Account, EncryptionAlgo};
use token::delete_token;

use crate::token::{add_token, generate_token};
pub mod models;
pub mod token;
use clap::Parser;

#[derive(Parser, PartialEq, Eq, Debug)]
enum Opt {
    Get {
        #[arg(long, short)]
        account: String,
    },

    Add {
        #[arg(long)]
        account_name: String,

        #[arg(long, short)]
        secret: String,

        #[arg(long, short)]
        time: u64,

        #[arg(long, short)]
        algorithm: EncryptionAlgo,

        #[arg(long)]
        digits: Option<usize>,

        #[arg(long)]
        skew: Option<u8>,
    },

    Delete {
        #[arg(long)]
        name: String,
    },
}

fn main() -> Result<()> {
    let args = Opt::parse();
    let mut accounts = Account::load_tokens("token.json".to_string()).unwrap();
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
        Opt::Add {
            account_name,
            secret,
            time,
            algorithm,
            digits,
            skew,
        } => {
            let new_account = Account {
                account_name,
                secret,
                time,
                algorithm,
                digits,
                skew,
            };
            add_token(&mut accounts, new_account, None)?;
            println!("Account added!");
        }
        Opt::Delete { name } => {
            delete_token(&mut accounts, name, None)?;
        }
    }
    // let path = std::env::args().nth(1).unwrap();
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{BufRead, Read},
    };

    use eyre::Result;
    use tempdir::TempDir;

    use crate::{
        models::{Account, EncryptionAlgo},
        token::{add_token, delete_token},
    };

    fn init_account() -> Result<(TempDir, String)> {
        let test_account = Account {
            account_name: "Haduba".into(),
            secret: "xyz".into(),
            time: 30,
            algorithm: EncryptionAlgo::SHA1,
            digits: None,
            skew: None,
        };
        let tmp_dir = TempDir::new("test_dir")?;
        let file_path = tmp_dir.path().join("token.json");
        let path = file_path.to_str().unwrap().to_string();
        File::create(path.clone())?;
        add_token(&mut Vec::new(), test_account.clone(), Some(path.clone()))?;
        Ok((tmp_dir, path))
    }

    #[test]
    fn user_adds_account() -> Result<()> {
        let test_account = Account {
            account_name: "Haduba".into(),
            secret: "xyz".into(),
            time: 30,
            algorithm: EncryptionAlgo::SHA1,
            digits: None,
            skew: None,
        };
        let (dir, path) = init_account()?;
        let loaded_accounts = Account::load_tokens(path.clone())?;
        dir.close()?;
        assert_eq!(loaded_accounts, vec![test_account]);
        Ok(())
    }

    #[test]
    fn user_deletes_account() -> Result<()> {
        let (dir, path) = init_account()?;
        delete_token(&mut Vec::new(), "Haduba".to_string(), Some(path.clone()))?;
        let loaded_accounts = Account::load_tokens(path.clone())?;
        dir.close()?;
        assert_eq!(loaded_accounts, vec![]);
        Ok(())
    }
}
