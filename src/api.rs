use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub struct HTTPClient {
    pub client: reqwest::Client,
    pub api: String,
    pub headers: HeaderMap,
}

#[derive(Serialize, Deserialize)]
struct SentMessage {
    content: String,
}

impl HTTPClient {
    pub fn new(token: String, ver: String, mut api: String) -> Self {
        let mut map = HeaderMap::new();
        map.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bot {}", token).parse().unwrap(),
        );
        map.insert(
            reqwest::header::USER_AGENT,
            format!("DiscordBot (example.com, 0.0.0)").parse().unwrap(),
        );
        api.push_str(&format!("v{}/", ver));

        Self {
            client: reqwest::Client::new(),
            api,
            headers: map,
        }
    }

    pub async fn send_msg(&self, cid: String, content: String) -> Result<(), Error> {
        let url = format!("{}channels/{}/messages", self.api, cid);
        let msg = SentMessage { content };
        self.client
            .post(url)
            .json(&msg)
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }
}
