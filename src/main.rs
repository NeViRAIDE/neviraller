use color_eyre::eyre::Result;

mod neovim_nightly;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    tui::run_term().await
}
