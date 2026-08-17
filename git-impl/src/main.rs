use std::path::PathBuf;

use clap::{Parser, Subcommand};
mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::CatFile {
            pretty_print,
            object_hash,
        }) => commands::cat_file::invoke(pretty_print, object_hash)?,
        Some(Commands::HashObject { write, file }) => commands::hash_object::invoke(write, file)?,
        None => {}
    }

    Ok(())
}
