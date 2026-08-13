use anyhow::Context;
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{
    ffi::CStr,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

#[derive(Debug)]
pub enum Kind {
    Blob,
    Commit,
    Tree,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Blob => write!(f, "blob"),
            Kind::Commit => write!(f, "commit"),
            Kind::Tree => write!(f, "tree"),
        }
    }
}

pub struct Object<R> {
    pub kind: Kind,
    pub expected_size: u64,
    pub reader: R,
}

impl Object<()> {
    pub fn blob_from_file(file: impl AsRef<Path>) -> anyhow::Result<Object<impl Read>> {
        let file = file.as_ref();
        let stat = std::fs::metadata(&file)
            .with_context(|| format!("file metadata: {}", file.display()))?;
        let file =
            std::fs::File::open(&file).with_context(|| format!("open {}", file.display()))?;

        Ok(Object {
            kind: crate::objects::Kind::Blob,
            expected_size: stat.len(),
            reader: file,
        })
    }

    pub fn read(object_hash: &String) -> anyhow::Result<Object<impl BufRead>> {
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

        let z = z.take(size as u64);

        Ok(Object {
            kind,
            expected_size: size as u64,
            reader: z,
        })
    }
}

impl<R> Object<R>
where
    R: Read,
{
    pub fn write(mut self, writer: impl Write) -> anyhow::Result<[u8; 20]> {
        let e = ZlibEncoder::new(writer, Compression::default());

        let mut writer = HashWriter {
            hasher: Sha1::new(),
            writer: e,
        };

        write!(writer, "{} {}\0", self.kind, self.expected_size)?;
        std::io::copy(&mut self.reader, &mut writer).context("stream file into blob")?;

        writer.writer.finish()?;
        let hash = writer.hasher.finalize();

        Ok(hash.into())
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

/*

pub struct LimitReader<R> {
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

*/
