use std::path::Path;

use anyhow::Context;

use crate::{
    torrent::Torrent,
    tracker::{TrackerRequest, TrackerResponse},
};

pub async fn invoke(torrent: impl AsRef<Path>) -> anyhow::Result<()> {
    let t = Torrent::new(torrent).context("parse torrent file")?;

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

    for peer in tresponse.peers.0 {
        println!("{peer}")
    }

    Ok(())
}
