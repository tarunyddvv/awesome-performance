use std::{
    ffi::CStr,
    io::{BufRead, BufReader, Read},
};

use anyhow::Context;
use flate2::read::ZlibDecoder;

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
    // cat-file: for blob is used for reading blobs
    pub fn read(object_hash: &str) -> anyhow::Result<Object<impl BufRead>> {
        let f = std::fs::File::open(format!(
            "../.git/objects/{}/{}",
            &object_hash[..2],
            &object_hash[2..]
        ))
        .context("open in .git/objects")?;
        let z = ZlibDecoder::new(f);
        let mut z = BufReader::new(z);

        let mut buf = Vec::new();

        // reading header into the buffer
        z.read_until(0, &mut buf)
            .context("reading the header of .git/objects inside buf")?;

        // INFO: blob <size>\0<content>
        let header = CStr::from_bytes_with_nul(&buf)
            .context("validating a nul terminated header from .git/objects")?
            .to_str()
            .context("header is not a valid UTF-8 encoded string")?;

        let (kind, size) = if let Some((kind, size)) = header.split_once(' ') {
            let size = size.parse::<usize>().context("parse the size of content")?;

            let kind = match kind {
                "blob" => Kind::Blob,
                "commit" => Kind::Commit,
                "tree" => Kind::Tree,
                _ => anyhow::bail!("not a valid header kind: '{kind}'"),
            };

            (kind, size)
        } else {
            anyhow::bail!("not a valid header from .git/objects");
        };

        // NOTE: this won't error if the decompressed file is too long, but will least not spam stdout and be vulnerable to a zipbomb.
        let z = z.take(size as u64);

        Ok(Object {
            kind,
            expected_size: size as u64,
            reader: z,
        })
    }
}
