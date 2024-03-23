# Peynir

A command-line tool for generating and managing Time-Based-One-Time Passwords(TOTP).
This tool allows you to add, delete and list tokens. The tokens are stored in ```tokens.json``` file located in the project directory.

## Installation 
Make sure you have Rust and Cargo installed on your system. You can install them from [Rust's official website.](https://www.rust-lang.org/)

Clone the repo
```
$ git clone https://github.com/zayutaha/Peynir
$ cd Peynir
```

## Usage
### Adding tokens
To add a token, use the following command
```
cargo run -- add --name=<AccountName> --secret=<SecretKey> --time=<TimeInSeconds>
```
Replace `<AccountName>` with the name of your account or service, `<SecretKey>` with the secret key for the token (encoded in base32), and `<TimeInSeconds>` with the time interval for the TOTP code in seconds.

## Deleting tokens
To delete a token, use the following command
```
cargo run -- delete --name=<AccountName>
``` 

## Listing Tokens
```
cargo run -- list 
```

# License

This project is licensed under the MIT License - see the LICENSE file for details.



