use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod objects;

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

        blob_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file: PathBuf,
    },
    LsTree {
        #[clap(long = "name-only")]
        name_only: bool,

        tree_hash: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::CatFile {
            pretty_print,
            blob_hash,
        }) => commands::cat_file::invoke(pretty_print, blob_hash)?,
        Some(Commands::HashObject { write, file }) => commands::hash_object::invoke(write, file)?,
        Some(Commands::LsTree {
            name_only,
            tree_hash,
        }) => commands::ls_tree::invoke(name_only, tree_hash)?,
        None => {}
    }

    Ok(())
}
