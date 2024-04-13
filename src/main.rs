use color_eyre::eyre::Result;

mod dependencies;
mod neovim_nightly;
mod package_manager;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    tui::run_term().await
}
