use std::{fs, path::PathBuf};

use crate::{music_backup_entry::MusicBackupEntry, utils};

pub struct BackupManager {
    path: PathBuf,
    backup: Vec<MusicBackupEntry>,
}

impl BackupManager {
    pub fn new(path: PathBuf) -> Result<BackupManager, String> {
        Ok(BackupManager {
            backup: serde_json::from_str(
                fs::read_to_string(&path)
                    .map_err(utils::error_to_string)?
                    .as_str(),
            )
            .map_err(utils::error_to_string)?,
            path,
        })
    }

    pub fn get_backup(&self) -> &Vec<MusicBackupEntry> {
        &self.backup
    }

    pub fn find(&self, artist: String) -> Vec<&MusicBackupEntry> {
        let mut result = Vec::new();

        for entry in &self.backup {
            if entry.artist == artist {
                result.push(entry);
            }
        }

        result
    }

    pub fn toggle_downloaded(&mut self, id: usize) {
        let entry = self.backup.iter_mut().find(|entry| entry.id == id);

        if let Some(entry) = entry {
            entry.downloaded = !entry.downloaded
        }
    }

    pub fn save(&self) -> Result<(), String> {
        fs::write(
            &self.path,
            serde_json::to_string_pretty(&self.backup).map_err(utils::error_to_string)?,
        )
        .map_err(utils::error_to_string)?;

        Ok(())
    }
}
