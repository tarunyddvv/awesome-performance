use std::path::Path;

use anyhow::Context;

use crate::torrent::Torrent;

pub fn invoke(torrent: impl AsRef<Path>) -> anyhow::Result<()> {
    let t = Torrent::new(torrent).context("parse torrent file")?;

    println!("Tracker URL: {}", t.announce);
    println!("Length: {}", t.length());

    let hash = t.info_hash().context("get torrent info hash")?;

    println!("Info Hash: {}", hex::encode(hash));

    println!("Piece Length: {}", t.info.plength);

    println!("Piece Hashes:");
    for phash in t.info.pieces.0 {
        println!("{}", hex::encode(phash));
    }
    Ok(())
}
