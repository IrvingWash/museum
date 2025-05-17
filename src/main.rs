#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "gui")]
mod gui;

fn main() -> Result<(), String> {
    #[cfg(feature = "cli")]
    cli::run();

    #[cfg(feature = "gui")]
    gui::run();

    Ok(())
}
