use anyhow::Context;
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    io::{BufRead, BufReader, Read},
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
