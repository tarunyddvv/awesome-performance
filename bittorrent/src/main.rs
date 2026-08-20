use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};

use crate::{commands::decode::decode_bencoded_value, torrent::Torrent};

mod commands;
mod download;
mod peer;
mod torrent;
mod tracker;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[clap(rename_all = "snake_case")]
enum Commands {
    Decode {
        encoded_value: String,
    },
    Info {
        torrent: PathBuf,
    },
    Peers {
        torrent: PathBuf,
    },
    Handshake {
        torrent: PathBuf,
        peer: String,
    },
    DownloadPiece {
        #[clap(short = 'o')]
        output: PathBuf,
        torrent: PathBuf,
        pindex: usize,
    },
    Download {
        #[clap(short = 'o')]
        output: PathBuf,
        torrent: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Decode { encoded_value }) => {
            let value = decode_bencoded_value(&encoded_value);
            println!("{}", value.0)
        }
        Some(Commands::Info { torrent }) => commands::info::invoke(torrent).await?,
        Some(Commands::Peers { torrent }) => commands::peers::invoke(torrent).await?,
        Some(Commands::Handshake { torrent, peer }) => {
            commands::handshake::invoke(torrent, peer).await?
        }
        Some(Commands::DownloadPiece {
            output,
            torrent,
            pindex,
        }) => commands::download_piece::invoke(output, torrent, pindex).await?,
        Some(Commands::Download { output, torrent }) => {
            let torrent = Torrent::read(torrent)
                .await
                .context("read the torrent file")?;
            torrent.print_tree();

            let files = torrent.download_all().await?;
            tokio::fs::write(
                output,
                files.into_iter().next().expect("always one file").bytes(),
            )
            .await?;
        }
        None => {}
    }

    Ok(())
}
