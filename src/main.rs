use color_eyre::eyre::Result;

mod config;
mod neovim_nightly;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    config::run_app().await
}
