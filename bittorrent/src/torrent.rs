#![allow(unused)]
use serde::{Deserialize, Serialize};

use crate::torrent::hashes::Pieces;

/// Metainfo files (also known as .torrent files) are bencoded dictionaries with the following keys:
#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    /// The URL of the tracker.
    pub announce: String,
    /// This maps to a dictionary, with keys described below.
    pub info: Info,
}

impl Torrent {
    pub fn length(&self) -> usize {
        match self.info.keys {
            Keys::SingleFile { length } => length,
            Keys::MultiFile { ref files } => files.iter().map(|f| f.length).sum(),
        }
    }
}

/// info dictionary
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    /// UTF-8 encoded string which is the suggested name to save the file (or directory) as.
    pub name: String,

    /// piece length maps to the number of bytes in each piece the file is split into.
    /// For the purposes of transfer, files are split into fixed-size pieces which are
    /// all the same length except for possibly the last one which may be truncated.
    /// piece length is almost always a power of two,
    /// most commonly 2 18 = 256 K (BitTorrent prior to version 3.2 uses 2 20 = 1 M as default).
    #[serde(rename = "piece length")]
    pub plength: usize,

    /// pieces maps to a string whose length is a multiple of 20.
    /// It is to be subdivided into strings of length 20,
    /// each of which is the SHA1 hash of the piece at the corresponding index.
    pub pieces: Pieces,

    /// There is also a key length or a key files, but not both or neither.
    #[serde(flatten)]
    pub keys: Keys,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Keys {
    /// If length is present then the download represents a single file,
    SingleFile { length: usize },
    /// otherwise it represents a set of files which go in a directory structure.
    MultiFile { files: Vec<File> },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    /// length - The length of the file, in bytes.
    pub length: usize,

    /// A list of UTF-8 encoded strings corresponding to subdirectory names,
    /// the last of which is the actual file name (a zero length list is an error case).
    pub path: Vec<String>,
}

mod hashes {
    use serde::Serialize;
    use serde::ser::Serializer;
    use serde::{
        Deserialize, Deserializer,
        de::{self, Visitor},
    };
    use std::fmt;
    use std::io::Error;

    #[derive(Debug)]
    pub struct Pieces(pub Vec<[u8; 20]>);

    struct PiecesVisitor;

    impl<'de> Visitor<'de> for PiecesVisitor {
        type Value = Pieces;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("pieces maps to a string whose length is a multiple of 20.")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() % 20 != 0 {
                return Err(E::custom("It is not subdivided into strings of length 20."));
            }

            Ok(Pieces(
                v.chunks_exact(20)
                    .map(|slice_20| slice_20.try_into().expect("[u8] != [u8; 20]"))
                    .collect::<_>(),
            ))
        }
    }

    impl<'de> Deserialize<'de> for Pieces {
        fn deserialize<D>(deserializer: D) -> Result<Pieces, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(PiecesVisitor)
        }
    }

    impl Serialize for Pieces {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let single_slice = self.0.concat();

            serializer.serialize_bytes(&single_slice)
        }
    }
}
