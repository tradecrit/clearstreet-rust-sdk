use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use crate::client::async_client::AsyncClient;
use crate::error::Error;
use crate::orders::OrderSide;
use crate::utils::parse_response;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub created_at: i64,
    pub account_id: String,
    pub account_number: String,
    pub trade_id: String,
    pub order_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: String,
    pub price: String,
    pub running_position: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListTradesResponse {
    pub data: Vec<Trade>,
    pub next_page_token: Option<String>,
}

#[cfg(feature = "async")]
pub async fn get_trade(client: &AsyncClient, trade_id: &str) -> Result<Trade, Error> {
    let api_url: &str = &client.client_options.api_url;
    let account_id: &str = &client.client_options.account_id;

    let url: String = format!("{api_url}/studio/v2/accounts/{account_id}/trades/{trade_id}");

    let request_builder: RequestBuilder = client.client.get(&url);
    let response: Response = request_builder.send().await?;

    parse_response::<Trade>(response).await
}

#[cfg(feature = "async")]
pub async fn list_trades(client: &AsyncClient) -> Result<ListTradesResponse, Error> {
    let api_url: &str = &client.client_options.api_url;
    let account_id: &str = &client.client_options.account_id;

    let url: String = format!("{api_url}/studio/v2/accounts/{account_id}/trades");

    let request_builder: RequestBuilder = client.client.get(&url);
    let response: Response = request_builder.send().await?;

    parse_response::<ListTradesResponse>(response).await
}
