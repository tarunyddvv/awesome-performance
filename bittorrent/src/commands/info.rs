use std::path::Path;

use anyhow::Context;
use sha1::{Digest, Sha1};

use crate::torrent::Torrent;

pub fn invoke(torrent: impl AsRef<Path>) -> anyhow::Result<()> {
    let f = std::fs::read(torrent).context("read the torrent file")?;
    let t: Torrent = serde_bencode::from_bytes(&f).context("parse torrent file")?;

    println!("Tracker URL: {}", t.announce);
    println!("Length: {}", t.length());

    let encoded_info = serde_bencode::to_bytes(&t.info).context("bencoding torrent info")?;

    let mut hasher = Sha1::new();
    hasher.update(encoded_info);
    let hash = hasher.finalize();

    println!("Info Hash: {}", hex::encode(hash));

    println!("Piece Length: {}", t.info.plength);

    println!("Piece Hashes:");
    for phash in t.info.pieces.0 {
        println!("{}", hex::encode(phash));
    }
    Ok(())
}
