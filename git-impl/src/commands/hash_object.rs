use anyhow::Context;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{io::Write, path::PathBuf};

pub fn invoke(write: bool, file: PathBuf) -> anyhow::Result<()> {
    // INFO: blob <size>\0<content>
    fn write_blob(writer: impl Write, file: PathBuf) -> anyhow::Result<String> {
        let stat =
            std::fs::metadata(&file).with_context(|| format!("file stat: {}", file.display()))?;

        let z = ZlibEncoder::new(writer, Compression::default());

        let mut writer = HashWriter {
            writer: z,
            hasher: Sha1::new(),
        };
        write!(writer, "blob {}\0", stat.len())?;
        let mut content = std::fs::File::open(file).context("open in the file content")?;
        std::io::copy(&mut content, &mut writer).context("write the content of the file")?;

        writer.writer.finish()?;
        let hash = writer.hasher.finalize();

        Ok(hex::encode(hash))
    }

    let hash = if write {
        let tmp = "temporary";
        let hash = write_blob(
            std::fs::File::create(tmp).context("write blob object to temporary file")?,
            file,
        )
        .context("write out blob object")?;

        std::fs::create_dir_all(format!("../.git/objects/{}", &hash[..2]))
            .context("create the hash dir")?;
        std::fs::copy(
            tmp,
            format!("../.git/objects/{}/{}", &hash[..2], &hash[2..]),
        )
        .context("copy out the content of temp file to the actual path")?;

        hash
    } else {
        write_blob(std::io::sink(), file)?
    };

    println!("{hash}");
    Ok(())
}

struct HashWriter<W> {
    writer: W,
    hasher: Sha1,
}

impl<W> std::io::Write for HashWriter<W>
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
