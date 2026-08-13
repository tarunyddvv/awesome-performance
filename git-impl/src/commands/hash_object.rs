use crate::objects::Object;
use anyhow::{Context, Ok};
use std::path::PathBuf;

pub fn invoke(write: bool, file: PathBuf) -> anyhow::Result<()> {
    let hash = if write {
        let temp = "temporary";
        let hash = Object::blob_from_file(file)
            .context("open blob input file")?
            .write(std::fs::File::create(temp)?)
            .context("stream file into blob")?;

        let hash = hex::encode(hash);

        std::fs::create_dir_all(format!("../.git/objects/{}/", &hash[..2]))?;
        std::fs::rename(
            temp,
            format!("../.git/objects/{}/{}", &hash[..2], &hash[2..]),
        )?;

        hash
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
