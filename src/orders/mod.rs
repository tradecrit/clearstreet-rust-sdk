use crate::error::ErrorType::HttpError;
use crate::Error;
use crate::error::BrokerApiError;
use crate::utils::parse_response;
use crate::utils;
use serde::{Deserialize, Serialize};
use crate::Client;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RemoteOrderState {
    Open,
    Rejected,
    Closed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RemoteOrderStatus {
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


impl Client {
    pub async fn create_order(&self, create_order_params: CreateOrderParams) {
        // --url https://api.clearstreet.io/studio/v2/accounts/1234abc/orders \
        let client = self.build_authenticated_client();
        let url = format!("{}/studio/v2/accounts/{}/orders", self.api_url, create_order_params.account_id);


        let request_builder = client.post(&url)
            .json(&create_order_params);

        let response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: CreateOrderResponse = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }
}