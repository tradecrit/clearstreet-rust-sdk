mod error;
mod orders;
mod authentication;
mod utils;
mod account;

use authentication::token_manager::TokenManager;
use error::Error;


pub struct Client {
    pub api_url: String,
    pub token_manager: TokenManager,
}

impl Client {
    pub fn new(client_id: String, client_secret: String, api_url: String) -> Self {
        let audience = "https://api.clearstreet.io".to_string();
        let token_manager = TokenManager::new(client_id, client_secret, api_url.clone(), audience);

        Self {
            api_url,
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
    pub fn new_with_token(api_url: String, token: String) -> Self {
        let token_manager = TokenManager::with_static_token(token);
        Self {
            api_url,
            token_manager,
        }
    }
}