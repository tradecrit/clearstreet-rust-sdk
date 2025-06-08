use crate::client::async_client::AsyncClient;
use crate::client::sync_client::SyncClient;
use crate::error::Error;
use crate::error::ErrorType::HttpError;
use crate::utils::{parse_response, parse_response_blocking};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub account_id: String,
    pub account_number: String,
    pub symbol: String,
    pub quantity: String,
    pub average_cost: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListPositionsResponse {
    pub data: Vec<Position>,
    pub next_page_token: Option<String>,
}

#[cfg(feature = "async")]
pub async fn get_position(client: &AsyncClient, symbol: &str) -> Result<Position, Error> {
    let url = format!(
        "{}/studio/v2/accounts/{}/positions/{}",
        client.client_options.api_url, client.client_options.account_id, symbol
    );

    let request_builder = client.client.get(&url);
    let response: Response = request_builder.send().await?;

    parse_response::<Position>(response).await
}

#[cfg(feature = "async")]
pub async fn list_positions(client: &AsyncClient) -> Result<ListPositionsResponse, Error> {
    let url = format!(
        "{}/studio/v2/accounts/{}/positions",
        client.client_options.api_url, client.client_options.account_id
    );

    let request_builder = client.client.get(&url);
    let response: Response = request_builder.send().await?;

    parse_response::<ListPositionsResponse>(response).await
}

#[cfg(feature = "sync")]
pub fn get_position_blocking(client: &SyncClient, symbol: &str) -> Result<Position, Error> {
    let url = format!(
        "{}/studio/v2/accounts/{}/positions/{}",
        client.client_options.api_url, client.client_options.account_id, symbol
    );

    let request_builder = client.client.get(&url);
    let response: reqwest::blocking::Response = request_builder
        .send()
        .map_err(|e| Error::new(HttpError, &e.to_string()))?;

    parse_response_blocking::<Position>(response)
}

#[cfg(feature = "sync")]
pub fn list_positions_blocking(client: &SyncClient) -> Result<ListPositionsResponse, Error> {
    let url = format!(
        "{}/studio/v2/accounts/{}/positions",
        client.client_options.api_url, client.client_options.account_id
    );

    let request_builder = client.client.get(&url);
    let response: reqwest::blocking::Response = request_builder
        .send()
        .map_err(|e| Error::new(HttpError, &e.to_string()))?;

    parse_response_blocking::<ListPositionsResponse>(response)
}
