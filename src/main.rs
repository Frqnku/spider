mod parse_args;
mod scraper;
mod rebuild_url;
use parse_args::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::new();
    println!("{:?}", cli);
    if let Err(err) = cli.check_path() {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    let response = reqwest::get(&cli.url)
        .await
        .map_err(|e| format!("Failed to fetch URL: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {}", e))?;

    let image_urls: Vec<String> = scraper::extract_image_urls(&cli.url, &response)
        .map_err(|e| format!("Failed to extract image URLs: {}", e))?;

    for url in image_urls {
        println!("Found image URL: {}", url);
    }

    let deeper_urls: Vec<String> = scraper::extract_deeper_urls(&cli.url, &response)
        .map_err(|e| format!("Failed to extract deeper URLs: {}", e))?;

    for url in deeper_urls {
        println!("Found deeper URL: {}", url);
    }

    Ok(())
}
