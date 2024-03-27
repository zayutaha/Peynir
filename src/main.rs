pub mod account;
pub mod cli;
pub mod models;

use eyre::eyre;
use std::{
    fs::OpenOptions,
    io::BufWriter,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use account::{display, load};
use clap::Parser;
use cli::Opt;
use eyre::{ContextCompat, Result};
use models::Account;

use crate::account::generate;

fn main() -> Result<()> {
    let args = Opt::parse();
    color_eyre::install()?;
    let mut accounts = load(String::from("accounts.json")).unwrap();
    match args {
        Opt::Get { name } => {
            let a = accounts
                .iter()
                .find(|x| x.name == name)
                .wrap_err("Account not found")?;
            let account = generate(a)?;
            println!("{account:?}");
        }
        Opt::Add { name, secret, time } => {
            let len = name.clone().len();
            let new_account = Account { name, secret, time };
            if len > 20 {
                return Err(eyre!(
                    "String length exceeds the allowed length of 20 characters"
                ));
            }
            if len < 3 {
                return Err(eyre!("String length must be atleast 3 characters"));
            }

            accounts.push(new_account);
            println!("Account added!");
        }
        Opt::Delete { name } => {
            let index = accounts
                .iter()
                .position(|x| x.name == name)
                .expect("Account should exist");
            accounts.remove(index);
            println!("{name:?} removed!");
        }
        Opt::List {} => loop {
            let mut accs: Vec<(String, String)> = Vec::new();
            for a in accounts.clone() {
                accs.push((a.name.clone(), generate(&a)?));
            }
            display(&accs)?;
            std::thread::sleep(Duration::from_secs(
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() % 30,
            ));
        },
    }
    save_accounts(&accounts, String::from("./accounts.json"))?;
    Ok(())
}

fn save_accounts(accounts: &Vec<Account>, path: String) -> Result<()> {
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

    use crate::{account::load, models::Account, save_accounts};

    fn init_dir() -> Result<(TempDir, String)> {
        let tmp_dir = TempDir::new("test_dir")?;
        let file_path = tmp_dir.path().join("accounts.json");
        let path = file_path.to_str().unwrap().to_string();
        Ok((tmp_dir, path))
    }

    #[test]
    fn user_creates_account() -> Result<()> {
        let account = Account {
            name: "John".to_string(),
            secret: "xyz".to_string(),
            time: 30,
        };
        let (tmp_dir, path) = init_dir()?;
        save_accounts(&vec![account.clone()], path.clone())?;
        assert_eq!(load(path)?, vec![account]);
        tmp_dir.close()?;
        Ok(())
    }
}
