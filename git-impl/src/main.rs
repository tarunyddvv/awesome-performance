use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        }) => git_impl::cat_file::invoke(pretty_print, object_hash)?,
        Some(Commands::HashObject { write, file }) => git_impl::hash_object::invoke(write, file)?,
        None => {}
    }

    Ok(())
}
