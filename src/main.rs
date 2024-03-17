use eyre::{ContextCompat, Result};
use token::{delete_token, Account, EncryptionAlgo};

use crate::token::{add_token, generate_token};
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
            add_token(&mut accounts, new_account)?;
            println!("Account added!");
        }
        Opt::Delete { name } => {
            delete_token(&mut accounts, name)?;
        }
    }
    // let path = std::env::args().nth(1).unwrap();
    Ok(())
}
