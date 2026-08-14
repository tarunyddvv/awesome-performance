use serde::{Deserialize, Serialize};

use crate::tracker::peers::Peers;

#[derive(Debug, Serialize)]
pub struct TrackerRequest {
    /// the info hash of the torrent
    /// 20 bytes long, will need to be URL encoded
    /// Note: this is NOT the hexadecimal representation, which is 40 bytes long
    // info_hash: [u8; 20],

    /// a unique identifier for your client
    /// A string of length 20 that you get to pick.
    peer_id: String,

    ///  the port your client is listening on
    // You can set this to 6881, you will not have to support this functionality during this challenge.
    port: u16,

    /// the total amount uploaded so far
    /// Since your client hasn't uploaded anything yet, you can set this to 0.
    uploaded: usize,

    /// the total amount downloaded so far
    /// Since your client hasn't downloaded anything yet, you can set this to 0.
    downloaded: usize,

    /// the number of bytes left to download
    /// Since you client hasn't downloaded anything yet, this'll be the total length of the file (you've extracted this value from the torrent file in previous stages)
    left: usize,

    /// whether the peer list should use the compact representation
    /// For the purposes of this challenge, set this to 1.
    compact: u8,
}

impl TrackerRequest {
    pub fn new(peer_id: String, left: usize) -> Self {
        Self {
            peer_id,
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left,
            compact: 1,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TrackerResponse {
    /// An integer, indicating how often your client should make a request to the tracker.
    /// You can ignore this value for the purposes of this challenge.
    // pub interval: u64,

    /// A string, which contains list of peers that your client can connect to.
    /// Each peer is represented using 6 bytes. The first 4 bytes are the peer's IP address and the last 2 bytes are the peer's port number.
    pub peers: Peers,
}

mod peers {
    use serde::{
        Deserialize, Deserializer,
        de::{self, Visitor},
    };
    use std::{
        fmt,
        net::{Ipv4Addr, SocketAddrV4},
    };

    #[derive(Debug)]
    pub struct Peers(pub Vec<SocketAddrV4>);
    struct PeersVisitor;

    impl<'de> Visitor<'de> for PeersVisitor {
        type Value = Peers;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                "A string, which contains list of peers that your client can connect to.",
            )
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() % 6 != 0 {
                return Err(E::custom(
                    "Each peer is represented using 6 bytes. The first 4 bytes are the peer's IP address and the last 2 bytes are the peer's port number.",
                ));
            }

            Ok(Peers(
                v.chunks_exact(6)
                    .map(|slice_6| {
                        SocketAddrV4::new(
                            Ipv4Addr::new(slice_6[0], slice_6[1], slice_6[2], slice_6[3]),
                            u16::from_be_bytes([slice_6[4], slice_6[5]]),
                        )
                    })
                    .collect::<_>(),
            ))
        }
    }

    impl<'de> Deserialize<'de> for Peers {
        fn deserialize<D>(deserializer: D) -> Result<Peers, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(PeersVisitor)
        }
    }
}
