use std::{io, path::PathBuf};

use clap::{Parser, Subcommand};
use museum::{backup_manager::BackupManager, music_backuper::MusicBackuper};

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Backup {
            music_path,
            save_path,
            ignore,
        } => Cli::backup(music_path, save_path, ignore)?,
        Command::Manage { path, command } => {
            let manager = Cli::create_manager(PathBuf::from(path))?;

            match command {
                ManageCommand::Find { artist } => Cli::find(manager, artist)?,
                ManageCommand::Toggle { id } => Cli::toggle_downloaded(manager, id)?,
                ManageCommand::List => Cli::list(manager),
                ManageCommand::ListQueued { from } => Cli::list_queued(manager, from)?,
            }
        }
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
    /// Manage the backup
    Manage {
        /// Path to the backup file
        #[arg(short, long)]
        path: String,

        #[command(subcommand)]
        command: ManageCommand,
    },
}

#[derive(Subcommand)]
enum ManageCommand {
    /// Find entries
    Find {
        /// Artist name
        artist: String,
    },
    /// Toggle downloaded status
    Toggle {
        /// Entry id
        id: usize,
    },
    /// List all entries
    List,
    /// List the entries one by one in interactive mode
    ListQueued { from: Option<String> },
}

impl Cli {
    fn backup(music_path: String, save_path: String, ignore: Vec<String>) -> Result<(), String> {
        let mut backuper =
            MusicBackuper::new(PathBuf::from(music_path), PathBuf::from(save_path), ignore);

        backuper.backup()?;

        backuper.save()?;

        Ok(())
    }

    fn list(manager: BackupManager) {
        for entry in manager.get_backup() {
            println!("{}\n", entry)
        }
    }

    fn list_queued(mut manager: BackupManager, from: Option<String>) -> Result<(), String> {
        let mut backup = manager.get_backup().clone();
        backup.sort_by(|a, b| a.artist.cmp(&b.artist));

        if let Some(from) = from {
            if let Some(position) = backup
                .iter()
                .position(|entry| entry.artist.starts_with(&from))
            {
                backup = backup.split_off(position);
            }
        }

        println!("n - next, t - toggle, q - exit\n");

        for entry in backup {
            println!("{}", entry);

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to parse input");

            match input.trim() {
                "n" => continue,
                "q" => break,
                "t" => {
                    manager.toggle_downloaded(entry.id);
                    manager.save()?;
                }
                _ => eprintln!("Unknown input"),
            };
        }

        Ok(())
    }

    fn find(manager: BackupManager, artist: String) -> Result<(), String> {
        let found_entries = manager.find(artist);

        for entry in found_entries {
            println!("{}\n", entry);
        }

        Ok(())
    }

    fn toggle_downloaded(mut manager: BackupManager, id: usize) -> Result<(), String> {
        manager.toggle_downloaded(id);

        manager.save()
    }

    fn create_manager(path: PathBuf) -> Result<BackupManager, String> {
        BackupManager::new(path)
    }
}
