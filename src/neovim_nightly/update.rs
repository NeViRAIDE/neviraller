use std::process::Command;

pub async fn update_neovim() -> Result<(), Box<dyn std::error::Error>> {
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
