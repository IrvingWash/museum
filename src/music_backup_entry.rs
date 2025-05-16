use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MusicBackupEntry {
    pub id: usize,
    pub artist: String,
    pub albums: Vec<String>,
    pub downloaded: bool,
}

impl Display for MusicBackupEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\nartist: {}\nalbums: {}\ndownloaded: {}",
            self.id,
            self.artist,
            self.albums.join(", "),
            self.downloaded
        )
    }
}
