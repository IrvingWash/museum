use std::io::Write;
use std::{
    fs::{self, DirEntry, File},
    path::PathBuf,
};

use serde::Serialize;

use crate::utils;

pub struct MusicBackuper {
    backup: Vec<MusicBackupEntry>,
    path: PathBuf,
    save_path: PathBuf,
    ignore: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct MusicBackupEntry {
    pub artist: String,
    pub albums: Vec<String>,
}

impl MusicBackuper {
    pub fn new(path: PathBuf, save_path: PathBuf, ignore: Vec<String>) -> MusicBackuper {
        MusicBackuper {
            backup: Vec::with_capacity(300),
            path,
            ignore,
            save_path,
        }
    }

    pub fn backup(&mut self) -> Result<(), String> {
        let music_dir = fs::read_dir(&self.path).map_err(utils::error_to_string)?;

        for artist_dir in music_dir {
            let artist_dir = artist_dir.map_err(utils::error_to_string)?;

            if self.ignore_if_needed(&artist_dir)? {
                continue;
            };

            let mut entry = MusicBackupEntry {
                artist: artist_dir.file_name().to_string_lossy().to_string(),
                albums: Vec::new(),
            };

            let artist_dir = fs::read_dir(artist_dir.path()).map_err(utils::error_to_string)?;

            for album_dir in artist_dir {
                let album_dir = album_dir.map_err(utils::error_to_string)?;

                if self.ignore_if_needed(&album_dir)? {
                    continue;
                }

                entry
                    .albums
                    .push(album_dir.file_name().to_string_lossy().to_string());
            }

            self.backup.push(entry);
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        fs::create_dir_all(
            self.save_path
                .parent()
                .ok_or("Failed to create save file path")?,
        )
        .map_err(utils::error_to_string)?;

        let mut backup_file = File::create(&self.save_path).map_err(utils::error_to_string)?;

        backup_file
            .write(
                serde_json::to_string_pretty(&self.backup)
                    .map_err(utils::error_to_string)?
                    .as_bytes(),
            )
            .map_err(utils::error_to_string)?;

        Ok(())
    }

    fn ignore_if_needed(&self, dir_entry: &DirEntry) -> Result<bool, String> {
        if self
            .ignore
            .contains(&dir_entry.file_name().to_string_lossy().to_string())
        {
            return Ok(true);
        }

        if dir_entry
            .file_type()
            .map_err(utils::error_to_string)?
            .is_file()
        {
            println!(
                "Skipped a file while parsing artists: {}",
                dir_entry.path().to_string_lossy()
            );

            return Ok(true);
        }

        Ok(false)
    }
}
