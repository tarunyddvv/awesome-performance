use crate::objects::{Kind, Object};
use anyhow::Context;

pub fn invoke(name_only: bool, tree_hash: String) -> anyhow::Result<()> {
    anyhow::ensure!(name_only, "-name-only subcommand is mandatory");

    let mut object =
        Object::read(tree_hash).context("reading the object header to parse header and content")?;

    match object.kind {
        Kind::Tree => {
            let mut stdout = std::io::stdout().lock();
            let n = std::io::copy(&mut object.reader, &mut stdout)
                .context("writing the content of file from reader to stdout")?;

            anyhow::ensure!(
                n == object.expected_size,
                "invalid file size (actual: {n}, expected: {})",
                object.expected_size
            );
        }
        _ => anyhow::bail!("we do not yet know how to print: {}", object.kind),
    }

    Ok(())
}
