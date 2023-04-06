use serde::{Deserialize, Serialize};

pub(crate) struct MinifluxContext {
    token: String
}

const BASE: &str = "https://miniflux.privacy.qvarford.net";

impl MinifluxContext {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }

    pub(crate) fn fetch_feeds(&self) -> Vec<Feed> {
        use reqwest::blocking as reqwest;

        let client = reqwest::Client::new();
        let url = format!("{BASE}/v1/feeds");
        let response = client.get(url)
            .header("X-Auth-Token", &self.token)
            .send()
            .unwrap();
        let status = response.status();
        println!("{status}");
        assert!(status.is_success());

        serde_json::from_str(&response.text().unwrap()).unwrap()
    }

    pub(crate) fn update_feed(&self, feed: Feed) {
        use reqwest::blocking as reqwest;

        let client = reqwest::Client::new();
        let id = feed.id;
        let url = format!("{BASE}/v1/feeds/{id}");
        let request_body = serde_json::to_string(&feed).unwrap();

        let response = client.put(url)
            .header("X-Auth-Token", &self.token)
            .body(request_body)
            .send()
            .unwrap();
        let status = response.status();
        println!("{status}");
        assert!(status.is_success());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Feed {
    pub id: i64,
    user_id: u64,
    title: String,
    site_url: String,
    pub feed_url: String,
    checked_at: String,
    etag_header: String,
    last_modified_header: String,
    parsing_error_message: String,
    parsing_error_count: u64,
    pub scraper_rules: String,
    pub rewrite_rules: String,
    crawler: bool,
    blocklist_rules: String,
    keeplist_rules: String,
    user_agent: String,
    username: String,
    password: String,
    disabled: bool,
    ignore_http_cache: bool,
    fetch_via_proxy: bool,
    category: Category,
    icon: Option<Icon>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    id: u64,
    user_id: u64,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Icon {
    feed_id: u64,
    icon_id: u64,
}