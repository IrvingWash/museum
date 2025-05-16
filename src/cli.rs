use std::path::PathBuf;

use clap::{Parser, Subcommand};
use museum::music_backuper::MusicBackuper;

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
        /// Entries to ignore
        #[arg(short, long, value_delimiter = ',')]
        ignore: Vec<String>,
    },
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }

    pub fn backup(
        music_path: String,
        save_path: String,
        ignore: Vec<String>,
    ) -> Result<(), String> {
        let mut backuper =
            MusicBackuper::new(PathBuf::from(music_path), PathBuf::from(save_path), ignore);

        backuper.backup()?;

        backuper.save()?;

        Ok(())
    }
}
