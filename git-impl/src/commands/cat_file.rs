use anyhow::Context;

use crate::{
    commands,
    objects::{Kind, Object},
};

// cat-file: for blob is used for reading blobs
pub fn invoke(pretty_print: bool, blob_hash: &String) -> anyhow::Result<()> {
    anyhow::ensure!(pretty_print, "-p (pretty print) subcommand is mandatory");

    let mut object = Object::read(&blob_hash).context("parse out the blob object hash")?;

    match object.kind {
        Kind::Blob => {
            let mut stdout = std::io::stdout().lock();
            let n = std::io::copy(&mut object.reader, &mut stdout)
                .context("copy the content the header from z reader to stdout stream")?;
            anyhow::ensure!(
                n == object.expected_size,
                "invalid content size: (actual: '{n}', expected: '{}')",
                object.expected_size
            );
        }
        Kind::Tree => commands::ls_tree::invoke(!pretty_print, &blob_hash)?,
        _ => anyhow::bail!("we do not yet know how to print '{}'", object.kind),
    }

    Ok(())
}
