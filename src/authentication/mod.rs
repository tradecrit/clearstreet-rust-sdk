use std::fmt::Debug;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::error::{Error};
use serde::{Deserialize, Serialize};
use crate::error::ErrorType::HttpError;
use crate::client::async_client::AsyncClient;
use crate::client::sync_client::SyncClient;
use crate::utils::{parse_response, parse_response_blocking};

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

#[cfg(feature = "async")]
pub async fn fetch_new_token(client: &AsyncClient) -> Result<TokenResponse, Error> {
    let body = TokenRequest {
        grant_type: "client_credentials".to_string(),
        client_id: client.client_options.client_id.clone(),
        client_secret: client.client_options.client_secret.clone(),
        audience: "https://api.clearstreet.io".to_string()
    };

    let url = "https://auth.clearstreet.io/oauth/token";

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert("user-agent", "clearstreet-sdk".parse()?);

    let response = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await?;
        return Err(Error::new(HttpError, &format!("Error: {} - {}", status, error_body)));
    }

    parse_response::<TokenResponse>(response).await
}

#[cfg(feature = "sync")]
pub fn fetch_new_token_blocking(client: &SyncClient) -> Result<TokenResponse, Error> {
    let body = TokenRequest {
        grant_type: "client_credentials".to_string(),
        client_id: client.client_options.client_id.clone(),
        client_secret: client.client_options.client_secret.clone(),
        audience: "https://api.clearstreet.io".to_string()
    };

    let url = "https://auth.clearstreet.io/oauth/token";

    let client = reqwest::blocking::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert("user-agent", "clearstreet-sdk".parse()?);

    let response = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text()?;
        return Err(Error::new(HttpError, &format!("Error: {} - {}", status, error_body)));
    }

    parse_response_blocking::<TokenResponse>(response)
}
