pub async fn check_neovim_version(
    new_version_line: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new("nvim")
        .arg("--version")
        .output()?;

    let current_version_output = std::str::from_utf8(&output.stdout)?;

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
