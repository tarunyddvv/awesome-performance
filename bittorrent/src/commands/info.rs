use std::path::Path;

use anyhow::Context;

use crate::torrent::Torrent;

pub fn invoke(torrent: impl AsRef<Path>) -> anyhow::Result<()> {
    let f = std::fs::read(torrent).context("read the torrent file")?;
    let t: Torrent = serde_bencode::from_bytes(&f).context("parse torrent file")?;

    println!("Tracker URL: {}", t.announce);
    println!("Length: {}", t.length());

    Ok(())
}
