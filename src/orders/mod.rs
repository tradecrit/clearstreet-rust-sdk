use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CreateOrderParams<'a> {
    pub account_id: &'a str,
    pub reference_id: Option<&'a str>,
    pub order_type: OrderType,
    #[serde(rename = "side")]
    pub order_side: OrderSide,
    pub quantity: &'a str, // very weird but it's a string
    pub price: Option<&'a str>, // very weird but it's a string
    pub stop_price: Option<&'a str>,
    pub time_in_force: Option<TimeInForce>,
    pub symbol: &'a str,
    pub symbol_format: SymbolFormat,
    pub routing_strategy: Option<StrategyType>
}
