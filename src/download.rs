use chrono::Local;
use dashmap::DashSet;
use futures_util::StreamExt;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

fn generate_filename(content_type: &str) -> Option<String> {
    let content_type_lower = content_type.to_lowercase();
    let normalized_type = match content_type_lower.as_str() {
        "image/jfif" | "image/pjpeg" => "image/jpeg",
        other => other,
    };

    let correct_ext = match normalized_type {
        "image/jpeg" | "image/jpg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "image/bmp" => "bmp",
        "image/svg+xml" | "image/svg" => "svg",
        "image/tiff" | "image/tif" => "tiff",
        "image/x-icon" | "image/vnd.microsoft.icon" => "ico",
        "image/heif" | "image/heic" => "heif",
        "image/avif" => "avif",
        "image/jp2" => "jp2",
        "image/x-png" => "png",
        _ => "bin",
    };

    if correct_ext == "bin" {
        return None;
    }

    Some(format!(
        "{}_{}.{correct_ext}",
        Local::now().format("%Y-%m-%d-%H%M%S"),
        Uuid::new_v4()
    ))
}

pub async fn download_images(
    image_urls: Vec<String>,
    download_path: &Path,
    downloaded: Arc<DashSet<String>>,
) -> Result<(), String> {
    for url in image_urls {
        if downloaded.contains(&url) {
            continue;
        }

        let response = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to download image from {url}: {e}"))?;

        downloaded.insert(url.clone());

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");

        let filename = match generate_filename(content_type) {
            Some(name) => name,
            None => continue,
        };

        let file_path = download_path.join(filename);
        let mut file = File::create(&file_path)
            .await
            .map_err(|e| format!("Failed to create file for {url}: {e}"))?;

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let data = chunk.map_err(|e| format!("Failed to read chunk from {url}: {e}"))?;
            file.write_all(&data)
                .await
                .map_err(|e| format!("Failed to write chunk to file for {url}: {e}"))?;
        }

        println!("Downloaded image from {url}");
    }
    Ok(())
}
