use crate::error::Error;
use crate::utils::{parse_response};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[cfg(feature="async")]
use crate::client::async_client::AsyncClient;
#[cfg(feature="sync")]
use crate::client::sync_client::SyncClient;
#[cfg(feature="sync")]
use crate::utils::parse_response_blocking;

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
    let api_url: &str = &client.client_options.api_url;
    let account_id: &str = &client.client_options.account_id;

    let url = format!("{api_url}/studio/v2/accounts/{account_id}/positions/{symbol}");

    let request_builder = client.client.get(&url);
    let response: Response = request_builder.send().await?;

    parse_response::<Position>(response).await
}

#[cfg(feature = "async")]
pub async fn list_positions(client: &AsyncClient) -> Result<ListPositionsResponse, Error> {
    let api_url: &str = &client.client_options.api_url;
    let account_id: &str = &client.client_options.account_id;

    let url = format!("{api_url}/studio/v2/accounts/{account_id}/positions");

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
        .send()?;

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
        .send()?;

    parse_response_blocking::<ListPositionsResponse>(response)
}
