use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod commands;
pub mod objects;

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
    LsTree {
        #[clap(long)]
        name_only: bool,

        tree_hash: String,
    },
    WriteTree,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::CatFile {
            pretty_print,
            object_hash,
        }) => commands::cat_file::invoke(pretty_print, object_hash)?,
        Some(Commands::HashObject { write, file }) => commands::hash_object::invoke(write, &file)?,
        Some(Commands::LsTree {
            name_only,
            tree_hash,
        }) => commands::ls_tree::invoke(name_only, tree_hash)?,
        Some(Commands::WriteTree) => commands::write_tree::invoke()?,
        None => {}
    }

    Ok(())
}
