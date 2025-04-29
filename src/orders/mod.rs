use reqwest::{RequestBuilder, Response};
use crate::error::ErrorType::HttpError;
use crate::Error;
use crate::error::BrokerApiError;
use crate::utils::parse_response;
use crate::utils;
use serde::{Deserialize, Serialize};
use crate::Client;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderState {
    Open,
    Rejected,
    Closed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Replaced,
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    PendingNew,
    Calculated,
    Expired,
    AcceptedForBidding,
    PendingReplace,
    DoneForDay,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    #[serde(rename = "stop-limit")]
    StopLimit,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderSide {
    Market,
    Limit,
    Stop,
    #[serde(rename = "stop-limit")]
    StopLimit,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "day-plus")]
    DayPlus,
    #[serde(rename = "at-open")]
    AtOpen,
    #[serde(rename = "at-close")]
    AtClose,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SymbolFormat {
    #[serde(rename = "osi")]
    Osi,
    #[serde(rename = "cms")]
    Cms,
}

impl Default for SymbolFormat {
    fn default() -> Self {
        SymbolFormat::Cms
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Strategy {
    #[serde(rename = "type")]
    pub strategy_type: StrategyType,
    pub destination: Destination,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StrategyType {
    #[serde(rename = "sor")]
    SmartOrderRoute, // Smart Order Router
    #[serde(rename = "dark")]
    Dark, // Dark Pool
    #[serde(rename = "ap")]
    ArrivalPrice, // Arrival price
    #[serde(rename = "pov")]
    PercentageOfVolume, // Percentage of Volume
    #[serde(rename = "twap")]
    TimeWeightedAveragePrice, // Time Weighted Average Price
    #[serde(rename = "vwap")]
    VolumeWeightedAveragePrice, // Volume Weighted Average Price
    #[serde(rename = "dma")]
    DirectMarketAccess, // Direct Market Access
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Destination {
    ARCX, // NYSE ARCA
    BATS, // BATS Exchange
    BATY, // BATS Y Exchange
    EDGA, // EDGA Exchange
    EDGX, // EDGX Exchange
    EPRL, // MIAX Pearl Equities
    IEXG, // Investors' Exchange
    MEMX, // Members' Exchange
    XASE, // NYSE American
    XBOS, // NASDAQ BX Exchange
    XCIS, // NYSE National
    XNMS, // NASDAQ/NMS (Global Market)
    XNYS, // New York Stock Exchange
}

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
    pub time_in_force: Option<TimeInForce>,
    pub symbol: String,
    pub symbol_format: SymbolFormat,
    pub routing_strategy: Option<StrategyType>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderParams {
    pub account_id: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrdersParams {
    pub from: i64,
    pub to: i64,
    pub page_size: i64,
    pub page_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    pub quantity: String,
    pub price: Option<String>,
    pub stop_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub created_at: i64,
    pub updated_at: i64,
    pub order_id: String,
    pub reference_id: String,
    pub version: i64,
    pub account_id: String,
    pub account_number: String,
    pub state: OrderState,
    pub status: OrderStatus,
    pub symbol: String,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub quantity: String,
    pub price: String,
    pub stop_price: String,
    pub time_in_force: TimeInForce,
    pub average_price: i64,
    pub filled_quantity: String,
    pub order_update_reason: String,
    pub text: String,
    pub strategy: Strategy,
    pub running_position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetOrderResponse {
    pub order: Order,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetOrdersResponse {
    pub data: Vec<Order>,
    pub next_page_token: Option<String>,
}

impl Client {
    #[tracing::instrument(skip(self))]
    pub async fn create_order(&self, create_orders_params: CreateOrderParams) -> Result<CreateOrderResponse, Error> {
        tracing::debug!("create_order: {:?}", create_orders_params);

        let client = self.build_authenticated_client().await?;

        let url = format!("{}/studio/v2/accounts/{}/orders", self.api_url, create_orders_params.account_id);

        let request_builder: RequestBuilder = client.post(&url)
            .json(&create_orders_params);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: CreateOrderResponse = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_order(&self, orders_params: OrderParams) -> Result<Order, Error> {
        tracing::debug!("get_order: {:?}", orders_params);

        let client = self.build_authenticated_client().await?;

        let url = format!("{}/studio/v2/accounts/{}/orders/{}", self.api_url, orders_params.account_id, orders_params.order_id);

        let request_builder: RequestBuilder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: GetOrderResponse = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body.order);
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete_order(&self, orders_params: OrderParams) -> Result<(), Error> {
        tracing::debug!("delete_order: {:?}", orders_params);

        let client = self.build_authenticated_client().await?;

        let url: String = format!("{}/studio/v2/accounts/{}/orders/{}", self.api_url, orders_params.account_id, orders_params.order_id);

        let request_builder: RequestBuilder = client.delete(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            return Ok(());
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_order(&self, orders_params: OrderParams, update_order_request: UpdateOrderRequest) -> Result<(), Error> {
        tracing::debug!("update_order: {:?}", orders_params);
        tracing::debug!("update_order_request: {:?}", update_order_request);

        let client = self.build_authenticated_client().await?;

        let url: String = format!("{}/studio/v2/accounts/{}/orders/{}", self.api_url, orders_params.account_id, orders_params.order_id);

        let request_builder: RequestBuilder = client.put(&url)
            .json(&update_order_request);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            return Ok(());
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_orders(&self, account_id: &str, get_orders_params: GetOrdersParams) -> Result<Vec<Order>, Error> {
        tracing::debug!("get_orders");

        let client = self.build_authenticated_client().await?;

        let url: String = format!("{}/studio/v2/accounts/{}/orders", self.api_url, account_id);

        let request_builder: RequestBuilder = client.get(&url)
            .query(&[("from", get_orders_params.from)])
            .query(&[("to", get_orders_params.to)])
            .query(&[("page_size", get_orders_params.page_size)])
            .query(&[("page_token", get_orders_params.page_token)]);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: Vec<Order> = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }
}