use anyhow::Context;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{io::Write, path::PathBuf};

pub fn invoke(write: bool, file: PathBuf) -> anyhow::Result<()> {
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

        std::fs::create_dir_all(format!("../.git/objects/{}/", &hash[..2]))?;
        std::fs::rename(
            temp,
            format!("../.git/objects/{}/{}", &hash[..2], &hash[2..]),
        )?;

        hash
    } else {
        write_blob(std::io::sink(), file)?
    };

    println!("{hash}");

    Ok(())
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
