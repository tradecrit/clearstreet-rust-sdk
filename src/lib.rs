mod error;
mod orders;
mod authentication;
mod utils;
mod account;
mod instruments;
mod positions;
mod websockets;
mod trades;

use serde::{Deserialize, Serialize};
use authentication::token_manager::TokenManager;
use error::Error;

#[derive(Debug)]
pub struct Client {
    pub client_options: ClientOptions,
    pub token_manager: TokenManager, // not clone, otherwise your tokens will break
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientOptions {
    pub api_url: String,
    pub websocket_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            api_url: "https://api.clearstreet.io/studio/v2".to_string(),
            websocket_url: "wss://api.clearstreet.io/studio/v2/ws".to_string(),
            client_id: "<your_client_id>".to_string(),
            client_secret: "<your_client_secret>".to_string(),
        }
    }
}


impl Client {
    pub fn new(client_options: ClientOptions) -> Self {
        let token_manager: TokenManager = TokenManager::new(
            client_options.client_id.clone(),
            client_options.client_secret.clone(),
            client_options.api_url.clone(),
            client_options.api_url.clone()
        );

        Self {
            client_options,
            token_manager
        }
    }

    pub async fn build_authenticated_client(&self) -> Result<reqwest::Client, Error> {
        let token = self.token_manager.get_token().await?;

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

impl Client {
    pub fn new_with_token(token: String) -> Self {
        let token_manager = TokenManager::with_static_token(token);

        Self {
            client_options: ClientOptions::default(),
            token_manager,
        }
    }
}
