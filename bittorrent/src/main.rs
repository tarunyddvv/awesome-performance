use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::decode::decode_bencoded_value;

mod commands;
mod torrent;
mod tracker;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Decode { encoded_value: String },
    Info { torrent: PathBuf },
    Peers { torrent: PathBuf },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Decode { encoded_value }) => {
            let value = decode_bencoded_value(&encoded_value);
            println!("{}", value.0)
        }
        Some(Commands::Info { torrent }) => commands::info::invoke(torrent)?,
        Some(Commands::Peers { torrent }) => commands::peers::invoke(torrent).await?,
        None => {}
    }

    Ok(())
}
