use std::{fs, path::PathBuf};

use serde::Serialize;

use crate::utils;

#[derive(Debug, Serialize)]
pub struct MusicBackupEntry {
    pub artist: String,
    pub albums: Vec<String>,
}

pub fn backup_music(
    music_dir_path: PathBuf,
    ignore: Vec<String>,
) -> Result<Vec<MusicBackupEntry>, String> {
    let music_dir = fs::read_dir(music_dir_path).map_err(utils::error_to_string)?;

    let mut result = Vec::with_capacity(300);

    for artist_dir in music_dir {
        let artist_dir = artist_dir.map_err(utils::error_to_string)?;

        if ignore.contains(&artist_dir.file_name().to_string_lossy().to_string()) {
            continue;
        }

        if artist_dir
            .file_type()
            .map_err(utils::error_to_string)?
            .is_file()
        {
            println!(
                "Skipped a file while parsing artists: {}",
                artist_dir.path().to_string_lossy()
            );

            continue;
        };

        let mut entry = MusicBackupEntry {
            artist: artist_dir.file_name().to_string_lossy().to_string(),
            albums: Vec::new(),
        };

        let artist_dir = fs::read_dir(artist_dir.path()).map_err(utils::error_to_string)?;

        for album_dir in artist_dir {
            let album_dir = album_dir.map_err(utils::error_to_string)?;

            if ignore.contains(&album_dir.file_name().to_string_lossy().to_string()) {
                continue;
            }

            if album_dir
                .file_type()
                .map_err(utils::error_to_string)?
                .is_file()
            {
                println!(
                    "Skipped a file while parsing albums: {}",
                    album_dir.path().to_string_lossy()
                );

                continue;
            }

            entry
                .albums
                .push(album_dir.file_name().to_string_lossy().to_string());
        }

        result.push(entry);
    }

    Ok(result)
}
