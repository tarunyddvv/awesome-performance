use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{torrent::Torrent, tracker::peers::Peers};

#[derive(Debug, Serialize)]
pub struct TrackerRequest {
    /// a unique identifier for your client
    peer_id: String,

    ///  the port your client is listening on
    port: u16,

    /// the total amount uploaded so far
    uploaded: usize,

    /// the total amount downloaded so far
    downloaded: usize,

    /// the number of bytes left to download
    left: usize,

    /// whether the peer list should use the compact representation
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
    // pub interval: u64,

    /// A string, which contains list of peers that your client can connect to.
    /// Each peer is represented using 6 bytes. The first 4 bytes are the peer's IP address and the last 2 bytes are the peer's port number.
    pub peers: Peers,
}

impl TrackerResponse {
    pub async fn fetch(t: &Torrent) -> anyhow::Result<Self> {
        let trequest = TrackerRequest::new(String::from("00112233445566778899"), t.length());
        let trequest =
            serde_urlencoded::to_string(&trequest).context("urlencoding the tracker request")?;

        let urlencoded_info = t
            .info_hash()
            .context("get torrent info hash")?
            .iter()
            .map(|b| format!("%{:02x}", b))
            .collect::<String>();

        let url = format!("{}?{}&info_hash={}", t.announce, trequest, urlencoded_info);

        let tracker_response = reqwest::get(url)
            .await
            .context("get request for the peers")?
            .error_for_status()
            .context("tracker returned an error response")?
            .bytes()
            .await
            .context("bytes response from the tracker url")?;

        let tresponse: TrackerResponse =
            serde_bencode::from_bytes(&tracker_response).context("parsing the peer response")?;

        Ok(tresponse)
    }
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
