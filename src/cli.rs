use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use museum::music_backuper;

use crate::utils;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Backup a directory of music to json
    Backup {
        /// Path to the music directory
        #[arg(short, long)]
        music_path: String,
        /// Path to pub the json file with the backup
        #[arg(short, long)]
        save_path: String,
    },
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }

    pub fn backup(music_path: String, save_path: String) -> Result<(), String> {
        let backup = music_backuper::backup_music(PathBuf::from(music_path))?;

        let save_path = PathBuf::from(save_path);

        fs::create_dir_all(
            save_path
                .parent()
                .ok_or("Failed to create save file path")?,
        )
        .map_err(utils::error_to_string)?;

        let mut backup_file = File::create(save_path).map_err(utils::error_to_string)?;

        backup_file
            .write(
                serde_json::to_string_pretty(&backup)
                    .map_err(utils::error_to_string)?
                    .as_bytes(),
            )
            .map_err(utils::error_to_string)?;

        Ok(())
    }
}
