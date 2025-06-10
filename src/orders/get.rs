use crate::error::Error;
use crate::orders::Order;
use crate::utils::{parse_response};
use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};

#[cfg(feature="async")]
use crate::client::async_client::AsyncClient;
#[cfg(feature="sync")]
use crate::client::sync_client::SyncClient;
#[cfg(feature="sync")]
use crate::utils::parse_response_blocking;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderResponse {
    pub order: Order,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderParams {
    pub account_id: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrdersResponse {
    pub data: Vec<Order>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrdersParams {
    pub from: i64,
    pub to: i64,
    pub page_size: i64,
    pub page_token: String,
}

#[cfg(feature = "async")]
pub async fn get_order(client: &AsyncClient, order_id: &str) -> Result<Order, Error> {
    let url = format!(
        "{}/studio/v2/accounts/{}/orders/{}",
        client.client_options.api_url, client.client_options.account_id, order_id
    );

    let request_builder: RequestBuilder = client.client.get(&url);
    let response: Response = request_builder.send().await?;

    parse_response::<Order>(response).await
}

#[cfg(feature = "async")]
pub async fn list_orders(
    client: &AsyncClient,
    params: ListOrdersParams,
) -> Result<ListOrdersResponse, Error> {
    tracing::debug!("get_orders");

    let url: String = format!(
        "{}/studio/v2/accounts/{}/orders",
        client.client_options.api_url, client.client_options.account_id
    );

    let request_builder: RequestBuilder = client
        .client
        .get(&url)
        .query(&[("from", params.from)])
        .query(&[("to", params.to)])
        .query(&[("page_size", params.page_size)])
        .query(&[("page_token", params.page_token)]);

    let response: Response = request_builder.send().await?;

    parse_response::<ListOrdersResponse>(response).await
}

#[cfg(feature = "sync")]
pub fn get_order_blocking(client: &SyncClient, order_id: &str) -> Result<Order, Error> {
    let url = format!(
        "{}/studio/v2/accounts/{}/orders/{}",
        client.client_options.api_url, client.client_options.account_id, order_id
    );

    let request_builder: reqwest::blocking::RequestBuilder = client.client.get(&url);
    let response: reqwest::blocking::Response = request_builder
        .send()?;
    
    parse_response_blocking::<Order>(response)
}

#[cfg(feature = "sync")]
pub fn list_orders_blocking(
    client: &SyncClient,
    params: ListOrdersParams,
) -> Result<ListOrdersResponse, Error> {
    tracing::debug!("get_orders");

    let url: String = format!(
        "{}/studio/v2/accounts/{}/orders",
        client.client_options.api_url, client.client_options.account_id
    );

    let request_builder: reqwest::blocking::RequestBuilder = client
        .client
        .get(&url)
        .query(&[("from", params.from)])
        .query(&[("to", params.to)])
        .query(&[("page_size", params.page_size)])
        .query(&[("page_token", params.page_token)]);

    let response: reqwest::blocking::Response = request_builder
        .send()?;

    parse_response_blocking::<ListOrdersResponse>(response)
}
