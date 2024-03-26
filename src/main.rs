use scraper::{Html, Selector};
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let version = scrap().await?;

    check_neovim_version(&version).await?;
    offer_update(&version).await?;

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
    let version = version.trim().replace("NVIM ", "");

    Ok(version)
}

async fn check_neovim_version(new_version_line: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("nvim").arg("--version").output()?;

    let current_version_output = str::from_utf8(&output.stdout)?;
    let current_version_line = current_version_output
        .lines()
        .next()
        .ok_or("Failed to get current Neovim version")?;
    let current_version = current_version_line
        .split_whitespace()
        .nth(1)
        .ok_or("Failed to parse current Neovim version")?;

    println!("Current installed Neovim version: {}", current_version);

    let new_version = new_version_line.lines().next().ok_or("problem1")?;

    if current_version == new_version {
        println!("You are already using the latest Neovim version");
        return Err("Already up to date".into());
    }
    println!(
        "There is a newer version of Neovim Nightly available: {}",
        new_version
    );

    Ok(())
}

async fn offer_update(new_version: &str) -> Result<(), Box<dyn std::error::Error>> {
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

async fn update_neovim() -> Result<(), Box<dyn std::error::Error>> {
    let nvim_url = "https://github.com/neovim/neovim/releases/download/nightly/nvim.appimage";
    let response = reqwest::get(nvim_url).await?;

    if response.status().is_success() {
        let path = "/tmp/nvim.appimage";
        let mut file = tokio::fs::File::create(path).await?;
        let content = response.bytes().await?;
        tokio::io::copy(&mut &content[..], &mut file).await?;

        Command::new("sudo")
            .arg("chmod")
            .arg("+x")
            .arg(path)
            .status()
            .expect("Failed to execute chmod");

        Command::new("sudo")
            .arg("mv")
            .arg(path)
            .arg("/usr/local/bin/nvim")
            .status()
            .expect("Failed to move file to /usr/local/bin");

        println!("Neovim Nightly has been updated successfully!");
    } else {
        eprintln!(
            "Failed to download Neovim Nightly. Error: {}",
            response.status()
        );
    }

    Ok(())
}
