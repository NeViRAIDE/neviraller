use std::io::{self, Write};

use crate::neovim_nightly::update::update_neovim;

pub async fn offer_update(new_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "A new version of Neovim Nightly is available: {}",
        new_version
    );
    println!("Would you like to update? (yes/no)");

    let mut user_input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut user_input)?;

    match user_input.trim().to_lowercase().as_str() {
        "yes" | "y" => {
            println!("Updating Neovim to the latest version...");
            update_neovim().await?;
        }
        "no" | "n" => println!("Update cancelled."),
        _ => println!("Invalid input. Update cancelled."),
    }

    Ok(())
}
