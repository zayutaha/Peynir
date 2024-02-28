use token::Token;

pub mod token;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let token = Token::load_token(path).unwrap();
    println!("{token:?}");
}
