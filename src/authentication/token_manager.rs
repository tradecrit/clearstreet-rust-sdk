use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, Instant};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use tokio::sync::RwLock;

use crate::error::{Error};
use serde::{Deserialize, Serialize};
use crate::error::ErrorType::HttpError;
use crate::utils;

/// Represents an access token and its expiration time.
#[derive(Debug, Clone)]
struct Token {
    access_token: String,
    expires_at: Instant,
}

/// Manages OAuth2 access tokens, including automatic refresh when expired.
pub struct TokenManager {
    client_id: String,
    client_secret: String,
    audience: String,
    token: Arc<RwLock<Option<Token>>>,
}

impl Debug for TokenManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenManager {{ client_id: **REDACTED**, client_secret: **REDACTED**, audience: {}, token: **REDACTED** }}", self.audience)
    }
}

/// Request body for fetching a new token.
#[derive(Serialize)]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    audience: String,
}

/// Response body when fetching a new token.
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

impl TokenManager {
    /// Creates a new `TokenManager` for managing dynamic access tokens.
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth2 client ID.
    /// * `client_secret` - OAuth2 client secret.
    /// * `api_url` - Base URL of the authentication server.
    /// * `audience` - API audience identifier.
    /// Asynchronously creates a new `TokenManager` and immediately fetches a token.
    pub async fn init(client_id: String, client_secret: String, audience: String) -> Result<Self, Error> {
        let manager = TokenManager {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
            audience: audience.clone(),
            token: Arc::new(RwLock::new(None)),
        };

        let token_response: TokenResponse = manager.fetch_new_token().await?;

        let new_token = Token {
            access_token: token_response.access_token.clone(),
            expires_at: Instant::now() + Duration::from_secs(token_response.expires_in.saturating_sub(60)),
        };

        *manager.token.write().await = Some(new_token);

        Ok(manager)
    }

    /// Retrieves a valid access token.
    ///
    /// If a valid token is already cached, it will return it.
    /// Otherwise, it will request a new token from the server.
    ///
    /// # Returns
    ///
    /// A valid access token as a `String`, or an error if the token could not be fetched.
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

        // Check again after acquiring write lock
        if let Some(token) = &*write_guard {
            if Instant::now() < token.expires_at {
                return Ok(token.access_token.clone());
            }
        }

        let response: TokenResponse = self.fetch_new_token().await?;

        let new_token = Token {
            access_token: response.access_token.clone(),
            expires_at: Instant::now() + Duration::from_secs(response.expires_in.saturating_sub(60)),
        };

        *write_guard = Some(new_token.clone());

        Ok(new_token.access_token)
    }

    /// Fetches a new access token from the authentication server.
    ///
    /// This method sends a client credentials grant request.
    async fn fetch_new_token(&self) -> Result<TokenResponse, Error> {
        let body = TokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            audience: self.audience.clone(),
        };

        let url = "https://auth.clearstreet.io/oauth/token";

        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let response = client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await?;
            return Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)));
        }

        let body: TokenResponse = utils::parse_response(response).await?;

        Ok(body)
    }

    /// Creates a `TokenManager` with a static token.
    ///
    /// Useful for testing or if you already have a valid long-lived token.
    ///
    /// # Arguments
    ///
    /// * `token` - A pre-existing access token.
    ///
    /// # Returns
    ///
    /// A `TokenManager` instance that always returns the provided token.
    pub fn with_static_token(token: String) -> Self {
        let static_token = Token {
            access_token: token,
            expires_at: Instant::now() + Duration::from_secs(60 * 60 * 8), // 8 hours
        };

        Self {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            audience: "".to_string(),
            token: Arc::new(RwLock::new(Some(static_token))),
        }
    }
}
