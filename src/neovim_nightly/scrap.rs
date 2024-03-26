pub async fn scrap() -> Result<String, Box<dyn std::error::Error>> {
    let html = reqwest::get("https://github.com/neovim/neovim/releases/tag/nightly")
        .await?
        .text()
        .await?;

    let document = scraper::Html::parse_document(&html);
    let version_selector = scraper::Selector::parse(".markdown-body pre code").unwrap();
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
