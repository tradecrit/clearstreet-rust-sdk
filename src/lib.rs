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
    pub async fn init(client_options: ClientOptions) -> Result<Self, Error> {
        let token_manager: TokenManager = TokenManager::init(
            client_options.client_id.clone(),
            client_options.client_secret.clone(),
            client_options.api_url.clone(),
        ).await?;

        let client = Self {
            client_options,
            token_manager,
        };

        Ok(client)
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
    pub fn new_with_token(api_url: String, websocket_url: String, token: String) -> Self {
        let token_manager = TokenManager::with_static_token(token);
        
        let client_options = ClientOptions {
            api_url,
            websocket_url,
            client_id: "<your_client_id>".to_string(),
            client_secret: "<your_client_secret>".to_string(),
            account_id: "<your_account_id>".to_string(),
        };

        Self {
            client_options,
            token_manager,
        }
    }
}
