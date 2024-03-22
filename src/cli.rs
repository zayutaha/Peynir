use clap::{arg, Parser};

use crate::models::EncryptionAlgo;

#[derive(Parser, PartialEq, Eq, Debug)]
pub enum Opt {
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
    List {},
}
