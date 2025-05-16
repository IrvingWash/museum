use cli::{Cli, Command};

mod cli;
mod utils;

fn main() -> Result<(), String> {
    let cli = Cli::new();

    match cli.command {
        Command::Backup {
            music_path,
            save_path,
        } => Cli::backup(music_path, save_path)?,
    };

    Ok(())
}
