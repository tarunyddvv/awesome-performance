use crate::torrent::Torrent;
use anyhow::Context;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[repr(C)]
pub struct Handshake {
    /// length of the protocol string (BitTorrent protocol) which is 19 (1 byte)
    pub length: u8,

    /// the string BitTorrent protocol (19 bytes)
    pub bittorrent: [u8; 19],

    /// eight reserved bytes, which are all set to zero (8 bytes)
    pub reserved: [u8; 8],

    /// sha1 infohash (20 bytes) (NOT the hexadecimal representation, which is 40 bytes long)
    pub info_hash: [u8; 20],

    /// peer id (20 bytes) (generate 20 random byte values)
    pub peer_id: [u8; 20],
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            length: 19,
            bittorrent: *b"BitTorrent protocol",
            reserved: [0; 8],
            info_hash,
            peer_id,
        }
    }

    pub fn as_bytes_mut<'a>(&'a mut self) -> &'a mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self as *mut Self as *mut u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

pub async fn invoke(torrent: impl AsRef<Path>, peer: String) -> anyhow::Result<()> {
    let t = Torrent::new(torrent).context("parse torrent file")?;
    let info_hash = t.info_hash().context("torrent info hash")?;

    let mut handshake = Handshake::new(info_hash, *b"00112233445566778899");

    let hanshake_bytes = handshake.as_bytes_mut();

    let mut peer = tokio::net::TcpStream::connect(peer)
        .await
        .context("connect to peer")?;

    peer.write_all(hanshake_bytes)
        .await
        .context("write handshake bytes to tcp stream")?;

    peer.read_exact(hanshake_bytes)
        .await
        .context("read peer from the stream")?;

    anyhow::ensure!(handshake.bittorrent == *b"BitTorrent protocol");
    anyhow::ensure!(handshake.length == 19);

    println!("Peer ID: {}", hex::encode(handshake.peer_id));

    Ok(())
}
