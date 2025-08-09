pub fn rebuild_url(base: &str, relative: &str) -> String {
    let mut url = url::Url::parse(base).expect("Invalid base URL");
    url.path_segments_mut().unwrap().pop();
    url.join(relative).expect("Failed to join URLs").to_string().replace("&amp;", "&")
}