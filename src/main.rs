mod cli;
mod gui;

fn main() -> Result<(), String> {
    gui::run();
    // cli::run()

    Ok(())
}
