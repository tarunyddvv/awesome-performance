use crate::{commands::handshake::Handshake, torrent::Torrent, tracker::TrackerResponse};
use anyhow::Context;
use bytes::{Buf, BytesMut};
use futures_util::stream::StreamExt;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::Decoder;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum MessageTag {
    Choke = 0,
    Unchoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
}

pub struct Message {
    tag: MessageTag,
    payload: Vec<u8>,
}

struct MessageFramer;

const MAX: usize = 1 << 16;

impl Decoder for MessageFramer {
    type Item = Message;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            // Not enough data to read length marker.
            return Ok(None);
        }

        // Read length marker.
        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&src[..4]);
        let length = u32::from_be_bytes(length_bytes) as usize;

        // Check that the length is not too large to avoid a denial of
        // service attack where the server runs out of memory.
        if length > MAX {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", length),
            ));
        }

        if src.len() < 4 + length {
            // The full string has not yet arrived.
            //
            // We reserve more space in the buffer. This is not strictly
            // necessary, but is a good idea performance-wise.
            src.reserve(4 + length - src.len());

            // We inform the Framed that we need more bytes to form the next
            // frame.
            return Ok(None);
        }

        // Use advance to modify src such that it no longer contains
        // this frame.
        let tag = match src[4] {
            0 => MessageTag::Choke,
            1 => MessageTag::Unchoke,
            2 => MessageTag::Interested,
            3 => MessageTag::NotInterested,
            4 => MessageTag::Have,
            5 => MessageTag::Bitfield,
            6 => MessageTag::Request,
            7 => MessageTag::Piece,
            8 => MessageTag::Cancel,
            tag => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("invalid message tag: {}", tag),
                ));
            }
        };

        let data = src[5..4 + length].to_vec();
        src.advance(4 + length);

        Ok(Some(Message { tag, payload: data }))
    }
}

pub async fn invoke(
    _output: impl AsRef<Path>,
    torrent: impl AsRef<Path>,
    _pindex: usize,
) -> anyhow::Result<()> {
    let t = Torrent::new(torrent).context("parse torrent file")?;
    let info_hash = t.info_hash().context("torrent info hash")?;

    let peer = TrackerResponse::fetch(&t)
        .await
        .context("get tracker response")?
        .peers
        .0[0];

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

    let mut peer = tokio_util::codec::Framed::new(peer, MessageFramer);

    let bitfield = peer
        .next()
        .await
        .context("first message is the bitfield message")?
        .expect("failed to get the bitfield message");

    anyhow::ensure!(bitfield.tag == MessageTag::Bitfield);

    println!("bitfield payload: {}", hex::encode(bitfield.payload));
    Ok(())
}
