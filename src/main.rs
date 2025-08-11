mod download;
mod extract_url;
mod parse_args;
mod rebuild_url;
mod scrape;
use std::{collections::HashSet, sync::Arc};

use parse_args::Cli;
use tokio::sync::{Mutex, Semaphore};

use crate::scrape::scrape_and_download;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::new();

    if let Err(err) = cli.check_path() {
        eprintln!("{err}");
        std::process::exit(1);
    }

    let semaphore = Arc::new(Semaphore::new(10)); // Limite 10 tâches simultanées
    let visited = Arc::new(Mutex::new(HashSet::new()));

    scrape_and_download(
        cli.url,
        cli.path,
        cli.recursive,
        cli.limit + 1,
        semaphore,
        visited,
    )
    .await?;

    Ok(())
}
