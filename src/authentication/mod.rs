use std::fmt::Debug;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::error::{Error};
use serde::{Deserialize, Serialize};
use crate::error::ErrorType::HttpError;
use crate::{utils, Client};

/// Represents an access token and its expiration time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_at: i64,
}

/// Request body for fetching a new token.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    audience: String,
}

/// Response body when fetching a new token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
}

impl Client {
    /// Fetches a new access token from the authentication server.
    ///
    /// This method sends a client credentials grant request.
    pub async fn fetch_new_token(&self) -> Result<TokenResponse, Error> {
        let body = TokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: self.client_options.client_id.clone(),
            client_secret: self.client_options.client_secret.clone(),
            audience: "https://api.clearstreet.io".to_string()
        };

        let url = "https://auth.clearstreet.io/oauth/token";

        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("user-agent", "clearstreet-sdk".parse().unwrap());

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
}
