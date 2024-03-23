use clap::{arg, Parser};

#[derive(Parser, PartialEq, Eq, Debug)]
pub enum Opt {
    Get {
        #[arg(long, short)]
        account: String,
    },

    Add {
        #[arg(long)]
        name: String,

        #[arg(long, short)]
        secret: String,

        #[arg(long, short)]
        time: u64,
    },

    Delete {
        #[arg(long)]
        name: String,
    },
    List {},
}
