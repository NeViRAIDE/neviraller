use self::neovim_nightly::scrap::scrap;
use self::neovim_nightly::update_offer::offer_update;
use self::neovim_nightly::ver_compare::check_neovim_version;

mod neovim_nightly;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = scrap().await?;

    check_neovim_version(&version).await?;
    offer_update(&version).await?;

    Ok(())
}
