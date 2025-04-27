
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::error::{BrokerApiError, Error};
use serde::{Deserialize, Serialize};
use crate::error::ErrorType::HttpError;
use crate::utils;

#[derive(Debug, Clone)]
struct Token {
    access_token: String,
    expires_at: Instant,
}

#[derive(Debug)]
pub struct TokenManager {
    client_id: String,
    client_secret: String,
    api_url: String,
    audience: String,
    token: Arc<RwLock<Option<Token>>>,
}

#[derive(Serialize)]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    audience: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

impl TokenManager  {
    pub fn new(client_id: String, client_secret: String, api_url: String, audience: String) -> Self {
        Self {
            client_id,
            client_secret,
            api_url,
            audience,
            token: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_token(&self) -> Result<String, Error> {
        {
            let read_guard = self.token.read().await;
            if let Some(token) = &*read_guard {
                if Instant::now() < token.expires_at {
                    return Ok(token.access_token.clone());
                }
            }
        }

        let mut write_guard = self.token.write().await;

        // Check again in case someone else refreshed while we were waiting for the write lock
        if let Some(token) = &*write_guard {
            if Instant::now() < token.expires_at {
                return Ok(token.access_token.clone());
            }
        }

        let response = self.fetch_new_token().await?;

        let new_token = Token {
            access_token: response.access_token.clone(),
            expires_at: Instant::now() + Duration::from_secs(response.expires_in.saturating_sub(60)),
        };

        *write_guard = Some(new_token.clone());

        Ok(new_token.access_token)
    }

    async fn fetch_new_token(&self) -> Result<TokenResponse, Error> {
        let body = TokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            audience: self.audience.clone(),
        };

        let url = format!("{}/oauth/token/", self.api_url);

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .header("user-agent", "TradeCrit")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            tracing::error!("Status: {:?}", response.status());

            let api_error: BrokerApiError = response.json().await?;

            tracing::error!("{:?}", api_error);

            return Err(Error::new(HttpError, api_error.to_string()));
        }

        let body: TokenResponse = utils::parse_response(response).await?;

        Ok(body)
    }
}

impl TokenManager {
    pub fn with_static_token(token: String) -> Self {
        let static_token = Token {
            access_token: token,
            expires_at: Instant::now() + Duration::from_secs(60 * 60 * 24), // 24 hours
        };

        Self {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            api_url: "".to_string(),
            audience: "".to_string(),
            token: Arc::new(RwLock::new(Some(static_token))),
        }
    }
}