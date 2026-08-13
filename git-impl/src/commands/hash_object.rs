use crate::objects::Object;
use anyhow::{Context, Ok};
use std::path::PathBuf;

pub fn invoke(write: bool, file: &PathBuf) -> anyhow::Result<()> {
    let object = Object::blob_from_file(file).context("open blob input file")?;
    let hash = if write {
        object
            .write_to_objects()
            .context("stream file into .git/objects")?
    } else {
        object
            .write(std::io::sink())
            .context("stream file into blob")?
    };
    let hash = hex::encode(hash);

    println!("{hash}");

    Ok(())
}
