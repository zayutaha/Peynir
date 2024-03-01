use token::Account;

use crate::token::generate_token;
pub mod token;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let accounts = Account::load_tokens(path).unwrap();
    for account in accounts {
        let token = generate_token(account).unwrap();
        println!("{:?}", token);
    }
}
