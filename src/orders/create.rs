use crate::error::{Error, ErrorType};
use crate::orders::strategy::Strategy;
use crate::orders::{OrderSide, OrderType, SymbolFormat, TimeInForce};
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
pub struct CreateOrderParams {
    pub account_id: String,
    pub reference_id: String,
    pub order_type: OrderType,
    #[serde(rename = "side")]
    pub order_side: OrderSide,
    pub quantity: String,
    pub price: Option<String>,
    pub stop_price: Option<String>,
    pub time_in_force: TimeInForce,
    pub symbol: String,
    pub symbol_format: SymbolFormat,
    pub strategy: Strategy,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

#[cfg(feature = "async")]
pub(crate) async fn create_order(
    async_client: &AsyncClient,
    params: CreateOrderParams,
) -> Result<CreateOrderResponse, Error> {
    let url = format!(
        "{}/studio/v2/accounts/{}/orders",
        async_client.client_options.api_url, async_client.client_options.account_id
    );

    let request_builder: RequestBuilder = async_client.client.post(&url).json(&params);

    let response: Response = request_builder.send().await?;

    let status = response.status();

    if !status.is_success() {
        let error_body = response.text().await?;
        return Err(Error::new(
            ErrorType::HttpError,
            &format!("Error: {} - {}", status, error_body),
        ));
    }

    parse_response::<CreateOrderResponse>(response).await
}

#[cfg(feature = "sync")]
pub(crate) fn create_order_blocking(
    sync_client: &SyncClient,
    params: CreateOrderParams,
) -> Result<CreateOrderResponse, Error> {
    let url = format!(
        "{}/studio/v2/accounts/{}/orders",
        sync_client.client_options.api_url, sync_client.client_options.account_id
    );

    let request_builder: reqwest::blocking::RequestBuilder =
        sync_client.client.post(&url).json(&params);

    let response: reqwest::blocking::Response = request_builder
        .send()?;

    let status = response.status();
    if !status.is_success() {
        let error_body = response.text()?;
        return Err(Error::new(
            ErrorType::HttpError,
            &format!("Error: {} - {}", status, error_body),
        ));
    }

    parse_response_blocking::<CreateOrderResponse>(response)
}
