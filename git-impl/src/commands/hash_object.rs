use crate::objects::Object;
use anyhow::{Context, Ok};
use std::path::PathBuf;

pub fn invoke(write: bool, file: &PathBuf) -> anyhow::Result<()> {
    let hash = if write {
        let hash = Object::blob_from_file(file)
            .context("open blob input file")?
            .write_to_objects()
            .context("stream file into .git/objects")?;

        hex::encode(hash)
    } else {
        let hash = Object::blob_from_file(file)
            .context("open blob input file")?
            .write(std::io::sink())
            .context("stream file into blob")?;

        hex::encode(hash)
    };

    println!("{hash}");

    Ok(())
}
