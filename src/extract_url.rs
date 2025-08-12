use url::Url;

use crate::rebuild_url::rebuild_url;

fn find_image_tags(html: &str) -> Vec<usize> {
    let tag = "<img";
    let mut start = 0;
    let mut indices = Vec::new();

    while let Some(pos) = html[start..].find(tag) {
        let found_at = start + pos;
        indices.push(found_at);
        start = found_at + tag.len();
    }

    indices
}

pub fn extract_image_urls(url: &str, html: &str) -> Result<Vec<String>, String> {
    let image_urls = find_image_tags(html)
        .into_iter()
        .filter_map(|index| {
            let src_start = html[index..].find("src=\"")? + index + 5;
            let src_end = html[src_start..].find("\"")? + src_start;
            let url = rebuild_url(url, &html[src_start..src_end]);
            Some(url)
        })
        .collect();

    Ok(image_urls)
}

fn find_a_href_tags(html: &str) -> Vec<usize> {
    let tag = "<a";
    let mut start = 0;
    let mut indices = Vec::new();

    while let Some(pos) = html[start..].find(tag) {
        let found_at = start + pos;
        indices.push(found_at);
        start = found_at + tag.len();
    }

    indices
}

pub fn extract_deeper_urls(url: &str, html: &str) -> Result<Vec<String>, String> {
    let base = Url::parse(url).map_err(|e| e.to_string())?;

    let a_href_urls = find_a_href_tags(html)
        .into_iter()
        .filter_map(|index| {
            let href_start = html[index..].find("href=\"")? + index + 6;
            let href_end = html[href_start..].find("\"")? + href_start;
            let raw_href = &html[href_start..href_end];

            let abs_url = rebuild_url(url, raw_href);

            if let Ok(parsed) = Url::parse(&abs_url) {
                if parsed.domain() == base.domain()
                    && parsed.path() != "/"
                {
                    dbg!(&abs_url);
                    return Some(abs_url);
                }
            }

            None
        })
        .collect();

    Ok(a_href_urls)
}