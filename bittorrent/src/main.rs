use clap::{Parser, Subcommand};

use crate::commands::decode::decode_bencoded_value;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Decode { encoded_value: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Decode { encoded_value }) => {
            let value = decode_bencoded_value(&encoded_value);
            println!("{}", value.0)
        }
        None => {}
    }
}
