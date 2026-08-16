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

// cat-file: for blob is used for reading blobs
pub fn invoke(pretty_print: bool, object_hash: String) -> anyhow::Result<()> {
    anyhow::ensure!(pretty_print, "-p (pretty print) subcommand is mandatory");

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
        .context("validating a nul terminated header from .git/objects")
        .context("failed to get the CStr")?
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

    let mut z = z.take(size as u64);

    match kind {
        Kind::Blob => {
            let mut stdout = std::io::stdout().lock();
            let n = std::io::copy(&mut z, &mut stdout)
                .context("copy the content the header from z reader to stdout stream")?;
            anyhow::ensure!(
                n == size as u64,
                "invalid content size: (actual: '{n}', expected: '{size}'"
            );
        }
        _ => anyhow::bail!("we do not yet know how to print '{kind}'"),
    }

    Ok(())
}
