#![allow(unused)]

use crate::{
    commands::{
        download_piece::{Message, MessageFramer, MessageTag},
        handshake::Handshake,
    },
    torrent::Torrent,
};
use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddrV4;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_util::codec::Framed;

pub struct Peer {
    addr: SocketAddrV4,
    stream: Framed<TcpStream, MessageFramer>,
}

impl Peer {
    pub async fn new(&self, addr: SocketAddrV4, info_hash: [u8; 20]) -> anyhow::Result<Self> {
        let mut peer = tokio::net::TcpStream::connect(self.addr)
            .await
            .context("connect to peer")?;

        let mut handshake = Handshake::new(info_hash, *b"00112233445566778899");

        let handshake_bytes = handshake.as_bytes_mut();

        peer.write_all(&handshake_bytes)
            .await
            .context("write handshake bytes to the stream")?;

        peer.read_exact(handshake_bytes)
            .await
            .context("read handshake bytes response from the peer")?;

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

        Ok(Self { addr, stream: peer })
    }

    pub async fn download(
        &mut self,
        piece_i: u32,
        block_i: u32,
        block_size: u32,
    ) -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}

#[repr(C)]
pub struct Request {
    index: [u8; 4],
    begin: [u8; 4],
    length: [u8; 4],
}

impl Request {
    pub fn new(index: u32, begin: u32, length: u32) -> Self {
        Self {
            index: u32::to_be_bytes(index),
            begin: u32::to_be_bytes(begin),
            length: u32::to_be_bytes(length),
        }
    }

    pub fn index(&self) -> u32 {
        u32::from_be_bytes(self.index)
    }

    pub fn begin(&self) -> u32 {
        u32::from_be_bytes(self.begin)
    }

    pub fn length(&self) -> u32 {
        u32::from_be_bytes(self.length)
    }

    pub fn as_bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}
