mod crypto;
mod storage;
mod models;
mod manager;
mod ui;

use std::io;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    io::stderr().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stderr());
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = ui::app::App::new()?;
    let res = ui::run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    io::stderr().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
        return Err(err);
    }

    Ok(())
}