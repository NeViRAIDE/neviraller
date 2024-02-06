// use chrono::prelude::*;
use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use std::process::Command;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let version = scrap().await?;
    println!("New Neovim Nightly version detected: {}", &version);

    let _ver = check_neovim_version(&version).await?;

    Ok(())
}

async fn scrap() -> Result<String, Box<dyn Error>> {
    let html = reqwest::get("https://github.com/neovim/neovim/releases/tag/nightly")
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);
    let version_selector = Selector::parse(".markdown-body pre code").unwrap();
    let version = document
        .select(&version_selector)
        .next()
        .ok_or("Couldn't find information about new version")?
        .text()
        .collect::<Vec<_>>()
        .join("");

    Ok(version)
}

async fn check_neovim_version(new_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("nvim").arg("--version").output()?;

    let current_version_output = str::from_utf8(&output.stdout)?;
    let current_version = current_version_output
        .lines()
        .next()
        .ok_or("Failed to get current Neovim version")?;

    println!("Current installed Neovim version: {}", current_version);

    if current_version.contains(new_version) {
        println!(
            "You are already using the latest Neovim version: {}",
            new_version
        );
    } else {
        println!(
            "There is a newer version of Neovim Nightly available: {}",
            new_version
        );
        // Здесь можно добавить логику для обновления Neovim
    }

    Ok(())
}
