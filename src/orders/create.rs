use crate::orders::ErrorType::HttpError;
use crate::error::Error;
use crate::orders::strategy::Strategy;
use crate::orders::{OrderSide, OrderType, SymbolFormat, TimeInForce};
use crate::utils::{parse_response, parse_response_blocking};
use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};

#[cfg(feature="async")]
use crate::client::async_client::AsyncClient;
#[cfg(feature="sync")]
use crate::client::sync_client::SyncClient;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiCreateOrderParams {
    account_id: String,
    reference_id: String,
    order_type: OrderType,
    #[serde(rename = "side")]
    order_side: OrderSide,
    quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<String>,
    time_in_force: TimeInForce,
    symbol: String,
    symbol_format: SymbolFormat,
    strategy: Strategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderParams {
    pub account_id: String,
    pub reference_id: String,
    pub order_type: OrderType,
    pub order_side: OrderSide,
    pub quantity: String,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub symbol: String,
    pub symbol_format: SymbolFormat,
    pub strategy: Strategy,
}

impl From<CreateOrderParams> for ApiCreateOrderParams {
    fn from(params: CreateOrderParams) -> Self {
        ApiCreateOrderParams {
            account_id: params.account_id,
            reference_id: params.reference_id,
            order_type: params.order_type,
            order_side: params.order_side,
            quantity: params.quantity,
            price: params.price.map(|p| p.to_string()),
            stop_price: params.stop_price.map(|sp| sp.to_string()),
            time_in_force: params.time_in_force,
            symbol: params.symbol,
            symbol_format: params.symbol_format,
            strategy: params.strategy,
        }
    }
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
    let api_params: ApiCreateOrderParams = params.into();

    let url = format!(
        "{}/studio/v2/accounts/{}/orders",
        async_client.client_options.api_url, async_client.client_options.account_id
    );

    let request_builder: RequestBuilder = async_client.client.post(&url).json(&api_params);
    let response: Response = request_builder.send().await?;

    parse_response::<CreateOrderResponse>(response).await
}

#[cfg(feature = "sync")]
pub(crate) fn create_order_blocking(
    sync_client: &SyncClient,
    params: CreateOrderParams,
) -> Result<CreateOrderResponse, Error> {
    let api_params: ApiCreateOrderParams = params.into();

    let url = format!(
        "{}/studio/v2/accounts/{}/orders",
        sync_client.client_options.api_url, sync_client.client_options.account_id
    );

    let request_builder: reqwest::blocking::RequestBuilder =
        sync_client.client.post(&url).json(&api_params);

    let response: reqwest::blocking::Response = request_builder
        .send()
        .map_err(|e| Error::new(HttpError, &e.to_string()))?;

    parse_response_blocking::<CreateOrderResponse>(response)
}
