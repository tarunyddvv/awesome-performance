use std::{
    ffi::CStr,
    io::{BufRead, BufReader, Read, Write},
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;

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

#[derive(Debug)]
enum Kind {
    Blob,
    Commit,
    Tree,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::CatFile {
            pretty_print,
            object_hash,
        }) => {
            anyhow::ensure!(pretty_print, "-p subcommand is mandatory");
            let f = std::fs::File::open(format!(
                "../.git/objects/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))
            .context("open in .git/objects")?;

            let z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);

            let mut buf = Vec::new();
            z.read_until(0, &mut buf)
                .context("reading until nul byte")?;

            let header = CStr::from_bytes_with_nul(&buf)
                .context("validating nul byte at EOF")?
                .to_str()
                .expect("not a valid UTF-8 encoded header");

            let (kind, size) = if let Some((kind, size)) = header.split_once(' ') {
                let kind = match kind {
                    "blob" => Kind::Blob,
                    "commit" => Kind::Commit,
                    "tree" => Kind::Tree,
                    kind => anyhow::bail!("not a valid header kind: '{kind}'"),
                };
                let size = size
                    .parse::<usize>()
                    .context("parsing the size of content to usize")?;

                (kind, size)
            } else {
                anyhow::bail!("not a valid header type");
            };

            buf.clear();
            buf.resize(size, 0);
            z.read_exact(&mut buf)
                .context("reading the actual content of the file")?;
            let n = z
                .read(&mut [0; 1])
                .context("validating EOF inside the content")?;

            anyhow::ensure!(n == 0, "content bytes has '{n}' trailing bytes");

            match kind {
                Kind::Blob => {
                    let mut stdout = std::io::stdout().lock();
                    stdout
                        .write_all(&buf)
                        .context("writing the content of the file to stdout")?;
                }
                _ => anyhow::bail!("we do not yet know how to print: {:#?}", kind),
            }
        }
        None => {}
    }

    Ok(())
}
