use color_eyre::eyre::Result;
use tokio::sync::mpsc;

use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

mod neovim_nightly;
mod tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stderr());
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel(100);
    tokio::spawn(async move {
        tui::app::event_handler(tx).await;
    });

    let res = tui::app::run_app(&mut terminal, rx).await;

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;

    res?;

    Ok(())
}
