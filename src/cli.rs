use std::path::PathBuf;

use clap::{Parser, Subcommand};
use museum::music_backuper::MusicBackuper;

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Backup {
            music_path,
            save_path,
            ignore,
        } => Cli::backup(music_path, save_path, ignore)?,
    }

    Ok(())
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
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
    fn backup(music_path: String, save_path: String, ignore: Vec<String>) -> Result<(), String> {
        let mut backuper =
            MusicBackuper::new(PathBuf::from(music_path), PathBuf::from(save_path), ignore);

        backuper.backup()?;

        backuper.save()?;

        Ok(())
    }
}
