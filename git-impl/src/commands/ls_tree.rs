use std::{
    ffi::CStr,
    io::{BufRead, Read, Write},
};

use anyhow::Context;

use crate::objects::{Kind, Object};

// cat-file: for blob is used for reading blobs
pub fn invoke(name_only: bool, tree_hash: &String) -> anyhow::Result<()> {
    // INFO: tree <size>\0
    // INFO: <mode> <name>\0<20_byte_sha>
    // INFO: <mode> <name>\0<20_byte_sha>
    let mut object = Object::read(&tree_hash).context("parse out the tree object hash")?;

    match object.kind {
        Kind::Tree => {
            let mut buf = Vec::new();
            let mut hash_bytes = [0u8; 20];
            let mut stdout = std::io::stdout().lock();

            loop {
                buf.clear();
                hash_bytes.fill(0);
                let n = object
                    .reader
                    .read_until(0, &mut buf)
                    .context("read the header of tree inside buf")?;
                if n == 0 {
                    break;
                }

                object
                    .reader
                    .read_exact(&mut hash_bytes)
                    .context("read the 20 byte sha inside buffer")?;

                let mode_and_name = CStr::from_bytes_with_nul(&buf)
                    .context("validate nul terminated mode and name")?;
                let mut bits = mode_and_name.to_bytes().splitn(2, |&b| b == b' ');
                let mode = bits.next().context("split always yields once")?;
                let name = bits
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("tree entry has no file name"))?;

                if name_only {
                    stdout
                        .write_all(name)
                        .context("write tree entry name to stdout")?;
                    writeln!(stdout).context("write tree entry newline")?;
                } else {
                    let mode = std::str::from_utf8(mode).context("mode is always valid utf-8")?;
                    let hash = hex::encode(hash_bytes);

                    let kind = Object::read(&hash)
                        .context("read the tree entry object")?
                        .kind;

                    write!(stdout, "{mode:0>6} {kind} {hash}    ")
                        .context("write tree entry hash to stdout")?;

                    stdout
                        .write_all(name)
                        .context("write tree entry name to stdout")?;
                    writeln!(stdout).context("write tree entry newline")?;
                }
            }
        }
        _ => anyhow::bail!("we do not yet know how to print '{}'", object.kind),
    }

    Ok(())
}
