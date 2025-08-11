use chrono::Local;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

fn generate_filename(content_type: &str) -> String {
    let content_type_lower = content_type.to_lowercase();
    let normalized_type = match content_type_lower.as_str() {
        "image/jfif" | "image/pjpeg" => "image/jpeg",
        other => other,
    };

    let correct_ext = match normalized_type {
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "image/bmp" => "bmp",
        _ => "jpg",
    };

    format!(
        "{}_{}.{correct_ext}",
        Local::now().format("%Y-%m-%d-%H%M%S"),
        Uuid::new_v4()
    )
}

pub async fn download_images(image_urls: Vec<String>, download_path: &Path) -> Result<(), String> {
    for url in image_urls {
        let response = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to download image from {url}: {e}"))?;

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");

        let filename = generate_filename(content_type);

        dbg!(&filename);

        let mut file = File::create(download_path.join(filename))
            .await
            .map_err(|e| format!("Failed to create file for {url}: {e}"))?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {e}"))?;
        file.write_all(&bytes)
            .await
            .map_err(|e| format!("Failed to write image to file: {e}"))?;
    }
    Ok(())
}
