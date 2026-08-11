use std::{
    ffi::CStr,
    io::Write,
    io::{BufRead, BufReader, Error, Read},
    path::PathBuf,
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};

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

#[derive(Debug)]
enum Kind {
    Blob,
    Commit,
    Tree,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
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

            // NOTE: this would not throw an error
            // let mut z = z.take(size as u64);

            let mut z = LimitReader {
                reader: z,
                limit: size,
            };

            match kind {
                Kind::Blob => {
                    let mut stdout = std::io::stdout().lock();
                    let n = std::io::copy(&mut z, &mut stdout)
                        .context("writing the content of file from reader to stdout")?;

                    anyhow::ensure!(
                        n == size as u64,
                        "invalid file size (actual: {n}, expected: {size})"
                    );
                }
                _ => anyhow::bail!("we do not yet know how to print: {:#?}", kind),
            }
        }
        Some(Commands::HashObject { write, file }) => {
            fn write_blob(writer: impl Write, file: PathBuf) -> anyhow::Result<String> {
                let stat = std::fs::metadata(&file)
                    .with_context(|| format!("file metadata: {}", file.display()))?;

                let e = ZlibEncoder::new(writer, Compression::default());

                let mut writer = HashWriter {
                    hasher: Sha1::new(),
                    writer: e,
                };
                let content = std::fs::read(&file).context("reading the contents of the file")?;
                write!(writer, "blob ")?;
                write!(writer, "{}\0", stat.len())?;
                writer.write_all(&content)?;

                writer.writer.finish()?;
                let hash = writer.hasher.finalize();

                Ok(hex::encode(hash))
            }

            let hash = if write {
                let temp = "temporary";
                let hash = write_blob(std::fs::File::create(temp)?, file)?;

                std::fs::create_dir(format!("../.git/objects/{}", &hash[..2]))?;
                std::fs::copy(
                    temp,
                    format!("../.git/objects/{}/{}", &hash[..2], &hash[2..]),
                )?;

                hash
            } else {
                write_blob(std::io::sink(), file)?
            };

            println!("{hash}");
        }
        None => {}
    }

    Ok(())
}

struct LimitReader<R> {
    reader: R,
    limit: usize,
}

impl<R> std::io::Read for LimitReader<R>
where
    R: Read,
{
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() > self.limit {
            buf = &mut buf[..self.limit + 1];
        }
        let n = self.reader.read(buf)?;
        if n > self.limit {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "content has '{n}' trailing bytes",
            ));
        }

        self.limit -= n;
        Ok(n)
    }
}

struct HashWriter<W> {
    hasher: Sha1,
    writer: W,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);

        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
