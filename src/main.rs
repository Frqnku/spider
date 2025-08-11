mod download;
mod parse_args;
mod rebuild_url;
mod scraper;
use parse_args::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::new();
    println!("{cli:?}");
    if let Err(err) = cli.check_path() {
        eprintln!("{err}");
        std::process::exit(1);
    }

    let response = reqwest::get(&cli.url)
        .await
        .map_err(|e| format!("Failed to fetch URL: {e}"))?
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {e}"))?;

    let image_urls: Vec<String> = scraper::extract_image_urls(&cli.url, &response)
        .map_err(|e| format!("Failed to extract image URLs: {e}"))?;

    if let Err(e) = download::download_images(image_urls, &cli.path).await {
        eprintln!("Error downloading images: {e}");
        std::process::exit(1);
    }

    let deeper_urls: Vec<String> = scraper::extract_deeper_urls(&cli.url, &response)
        .map_err(|e| format!("Failed to extract deeper URLs: {e}"))?;

    for url in deeper_urls {
        println!("Found deeper URL: {url}");
    }

    Ok(())
}
