pub mod error;
pub mod orders;
pub mod authentication;
pub mod utils;
pub mod account;
pub mod instruments;
pub mod positions;
pub mod websockets;
pub mod trades;

use serde::{Deserialize, Serialize};
use error::Error;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_options: ClientOptions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientOptions {
    pub api_url: String,
    pub websocket_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub account_id: String,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            api_url: "https://api.clearstreet.io".to_string(),
            websocket_url: "wss://api.clearstreet.io/studio/v2/ws".to_string(),
            client_id: "<your_client_id>".to_string(),
            client_secret: "<your_client_secret>".to_string(),
            account_id: "<your_account_id>".to_string(),
        }
    }
}


impl Client {
    pub fn new(client_options: ClientOptions) -> Self {
        Self {
            client_options
        }
    }

    pub async fn build_authenticated_client(&self, token: &str) -> Result<reqwest::Client, Error> {

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("authorization", format!("Bearer {}", token).parse().unwrap());
        headers.insert("accept", "application/json".parse().unwrap());
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert("user-agent", "TradeCrit".parse().unwrap());

        Ok(reqwest::Client::builder()
            .default_headers(headers)
            .build()?)
    }
}

