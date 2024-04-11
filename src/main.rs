use color_eyre::eyre::Result;
use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::mpsc;

use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

mod neovim_nightly;
mod tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stderr());
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel(100);

    // Изменилось: создание и запуск `App` теперь внутри `App::run`.
    let mut app = tui::app::App::new();
    tokio::spawn(async move {
        tui::app::event_handler(tx).await; // Обработчик событий теперь метод `App`, если это необходимо.
    });

    // Изменилось: используем метод `run` вместо `run_app`.
    app.run(&mut terminal, rx).await?;

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
