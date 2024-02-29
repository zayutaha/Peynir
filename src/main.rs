use token::Account;

use crate::token::generate_token;

pub mod token;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let account = Account::load_token(path).unwrap();
    let token = generate_token(account).unwrap();
    println!("{:?}", token);
}
