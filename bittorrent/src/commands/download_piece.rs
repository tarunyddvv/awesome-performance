use crate::{
    commands::handshake::Handshake, peer::Request, torrent::Torrent, tracker::TrackerResponse,
};
use anyhow::Context;
use bytes::{Buf, BufMut, BytesMut};
use futures_util::{SinkExt, stream::StreamExt};
use sha1::{Digest, Sha1};
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::{Decoder, Encoder};

const BLOCK_MAX: usize = 1 << 14;

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
        loop {
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
                src.reserve(4 + length - src.len());
                return Ok(None);
            }

            // A zero-length frame is a valid BitTorrent keep-alive message.
            if length == 0 {
                src.advance(4);
                continue;
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
                        std::io::ErrorKind::InvalidData,
                        format!("invalid message tag: {}", tag),
                    ));
                }
            };

            let data = src[5..4 + length].to_vec();
            src.advance(4 + length);

            return Ok(Some(Message { tag, payload: data }));
        }
    }
}

impl Encoder<Message> for MessageFramer {
    type Error = std::io::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Don't send a Message if it is longer than the other end will
        // accept.
        let length = 1 /* Message tag */ + item.payload.len() /* length of payload */;

        if length > MAX {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", length),
            ));
        }

        // Convert the length into a byte array.
        // The cast to u32 cannot overflow due to the length check above.
        let len_slice = u32::to_be_bytes(length as u32);

        // Reserve space in the buffer.
        dst.reserve(4 + 1 + item.payload.len());

        // Write the length, tag and payload to the buffer.
        dst.extend_from_slice(&len_slice);
        dst.put_u8(item.tag as u8);
        dst.extend_from_slice(&item.payload);
        Ok(())
    }
}

pub async fn invoke(
    output: impl AsRef<Path>,
    torrent: impl AsRef<Path>,
    pindex: usize,
) -> anyhow::Result<()> {
    let t = Torrent::read(torrent).await.context("parse torrent file")?;
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

    let mut peer = tokio_util::codec::Framed::new(peer, MessageFramer);

    let bitfield = peer
        .next()
        .await
        .context("peer closed the connection before sending a message")?
        .context("failed to decode the peer message")?;

    anyhow::ensure!(
        bitfield.tag == MessageTag::Bitfield,
        "expected a bitfield message, received {:?}",
        bitfield.tag
    );

    peer.send(Message {
        tag: MessageTag::Interested,
        payload: Vec::new(),
    })
    .await
    .context("send interested message")?;

    let unchoke = peer
        .next()
        .await
        .context("peer closed the connection before sending an unchoke message")?
        .context("failed to decode the peer response after interested")?;

    anyhow::ensure!(
        unchoke.tag == MessageTag::Unchoke,
        "expected an unchoke message, received {:?}",
        unchoke.tag
    );
    anyhow::ensure!(unchoke.payload.is_empty());

    anyhow::ensure!(
        pindex < t.info.pieces.0.len(),
        "piece index is out of range"
    );

    let piece_length = if pindex == t.info.pieces.0.len() - 1 {
        let remainder = t.length() % t.info.plength;
        if remainder == 0 {
            t.info.plength
        } else {
            remainder
        }
    } else {
        t.info.plength
    };
    let nblocks = piece_length.div_ceil(BLOCK_MAX);

    let mut buf = Vec::with_capacity(piece_length);
    for block in 0..nblocks {
        let begin = (block * BLOCK_MAX) as u32;
        let block_length = if block == nblocks - 1 {
            piece_length - block * BLOCK_MAX
        } else {
            BLOCK_MAX
        };

        let request = Request::new(pindex as u32, begin, block_length as u32);

        peer.send(Message {
            tag: MessageTag::Request,
            payload: request.as_bytes().to_vec(),
        })
        .await
        .context("send request message")?;

        let piece = peer
            .next()
            .await
            .context("peer closed the connection before sending an piece message")?
            .context("failed to decode the piece response after interested")?;

        anyhow::ensure!(
            piece.tag == MessageTag::Piece,
            "expected a piece message, received {:?}",
            piece.tag
        );

        anyhow::ensure!(
            piece.payload.len() >= 8,
            "piece message payload is too short: {} bytes",
            piece.payload.len()
        );

        let index = u32::from_be_bytes(piece.payload[..4].try_into().unwrap());
        let response_begin = u32::from_be_bytes(piece.payload[4..8].try_into().unwrap());
        let data = &piece.payload[8..];

        anyhow::ensure!(index == pindex as u32, "piece index does not match request");
        anyhow::ensure!(
            response_begin == begin,
            "piece block offset does not match request"
        );
        anyhow::ensure!(
            data.len() == block_length,
            "piece block has {} bytes, expected {}",
            data.len(),
            block_length
        );

        buf.extend_from_slice(data);
    }

    let actual_piece_hash = t.info.pieces.0[pindex];

    let mut hasher = Sha1::new();
    hasher.update(&buf);
    let expected_piece_hash = hasher.finalize();

    anyhow::ensure!(
        actual_piece_hash == expected_piece_hash,
        "actual and expected hashes did not match ( actual: '{}', expected: '{}'",
        hex::encode(actual_piece_hash),
        hex::encode(expected_piece_hash)
    );

    tokio::fs::write(output, buf)
        .await
        .context("write the content of piece to file")?;
    Ok(())
}
