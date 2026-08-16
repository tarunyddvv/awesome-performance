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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::CatFile {
            pretty_print,
            object_hash,
        }) => commands::cat_file::invoke(pretty_print, object_hash)?,
        None => {}
    }

    Ok(())
}
