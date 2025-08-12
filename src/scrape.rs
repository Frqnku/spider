use std::{collections::HashSet, path::PathBuf, sync::Arc};

use dashmap::DashSet;
use futures::future::{BoxFuture, FutureExt};
use tokio::sync::{Mutex, Semaphore};

use crate::{download, extract_url};

pub fn scrape_and_download(
    url: String,
    download_path: PathBuf,
    recursive: bool,
    limit: usize,
    semaphore: Arc<Semaphore>,
    visited: Arc<Mutex<HashSet<String>>>,
    downloaded: Arc<DashSet<String>>,
) -> BoxFuture<'static, Result<(), String>> {
    async move {
        {
            let mut visited_lock = visited.lock().await;
            if !visited_lock.insert(url.clone()) {
                return Ok(());
            }
        }

        if limit == 0 {
            return Ok(());
        }

        let _permit = semaphore.acquire().await.map_err(|e| e.to_string())?;

        let response = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to fetch URL {}: {}", &url, e))?
            .text()
            .await
            .map_err(|e| format!("Failed to read response text for {}: {}", &url, e))?;

        let image_urls = extract_url::extract_image_urls(&url, &response)?;
        download::download_images(image_urls, download_path.as_ref(), downloaded.clone()).await?;

        drop(_permit);

        if recursive && limit > 1 {
            let deeper_urls = extract_url::extract_deeper_urls(&url, &response)?;

            let mut handles = Vec::new();

            for deeper_url in deeper_urls {
                let download_path_clone = download_path.clone();
                let semaphore_clone = semaphore.clone();
                let visited_clone = visited.clone();
                let limit_clone = limit - 1;
                let downloaded_clone = downloaded.clone();

                let fut = scrape_and_download(
                    deeper_url,
                    download_path_clone,
                    recursive,
                    limit_clone,
                    semaphore_clone,
                    visited_clone,
                    downloaded_clone,
                );

                let handle = tokio::spawn(fut);
                handles.push(handle);
            }

            for handle in handles {
                let _ = handle.await;
            }
        }

        Ok(())
    }
    .boxed()
}
