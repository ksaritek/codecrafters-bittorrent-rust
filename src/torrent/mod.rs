use crate::hashes::Hashes;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    /// The URL of the tracker.
    pub announce: String,

    pub info: Info,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(rename = "piece length")]
    pub plength: usize,

    pub pieces: Hashes,

    #[serde(flatten)]
    pub keys: Keys,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Keys {
    SingleFile { length: usize },
    Multifile { files: Vec<File> },
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    pub length: usize,
    pub path: Vec<String>,
}
