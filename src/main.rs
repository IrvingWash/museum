use cli::{Cli, Command};

mod cli;

fn main() -> Result<(), String> {
    let cli = Cli::new();

    match cli.command {
        Command::Backup {
            music_path,
            save_path,
            ignore,
        } => Cli::backup(music_path, save_path, ignore)?,
    };

    Ok(())
}
