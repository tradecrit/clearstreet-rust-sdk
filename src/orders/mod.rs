use crate::error::{Error, ErrorType};
use crate::orders::strategy::Strategy;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod create;
pub mod delete;
pub mod get;
pub mod strategy;
pub mod update;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderState {
    Open,
    Rejected,
    Closed,
}

impl FromStr for OrderState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(OrderState::Open),
            "rejected" => Ok(OrderState::Rejected),
            "closed" => Ok(OrderState::Closed),
            _ => Error::internal("invalid OrderState")
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    New,
    #[serde(rename = "partially-filled")]
    PartiallyFilled,
    Filled,
    Canceled,
    Replaced,
    #[serde(rename = "pending-cancel")]
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    #[serde(rename = "pending-new")]
    PendingNew,
    Calculated,
    Expired,
    #[serde(rename = "accepted-for-bidding")]
    AcceptedForBidding,
    #[serde(rename = "pending-replace")]
    PendingReplace,
    #[serde(rename = "done-for-day")]
    DoneForDay,
}

impl FromStr for OrderStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new" => Ok(OrderStatus::New),
            "partially-filled" => Ok(OrderStatus::PartiallyFilled),
            "filled" => Ok(OrderStatus::Filled),
            "canceled" => Ok(OrderStatus::Canceled),
            "replaced" => Ok(OrderStatus::Replaced),
            "pending-cancel" => Ok(OrderStatus::PendingCancel),
            "stopped" => Ok(OrderStatus::Stopped),
            "rejected" => Ok(OrderStatus::Rejected),
            "suspended" => Ok(OrderStatus::Suspended),
            "pending-new" => Ok(OrderStatus::PendingNew),
            "calculated" => Ok(OrderStatus::Calculated),
            "expired" => Ok(OrderStatus::Expired),
            "accepted-for-bidding" => Ok(OrderStatus::AcceptedForBidding),
            "pending-replace" => Ok(OrderStatus::PendingReplace),
            "done-for-day" => Ok(OrderStatus::DoneForDay),
            other => Err(Error::new(
                ErrorType::ParseError,
                &format!("Invalid OrderStatus: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    #[serde(rename = "stop-limit")]
    StopLimit,
}

impl FromStr for OrderType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "market" => Ok(OrderType::Market),
            "limit" => Ok(OrderType::Limit),
            "stop" => Ok(OrderType::Stop),
            "stop-limit" => Ok(OrderType::StopLimit),
            other => Err(Error::new(
                ErrorType::ParseError,
                &format!("Invalid OrderType: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
    #[serde(rename = "sell-short")]
    SellShort,
}

impl FromStr for OrderSide {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy" => Ok(OrderSide::Buy),
            "sell" => Ok(OrderSide::Sell),
            "sell-short" => Ok(OrderSide::SellShort),
            other => Err(Error::new(
                ErrorType::ParseError,
                &format!("Invalid OrderSide: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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

impl FromStr for TimeInForce {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(TimeInForce::Day),
            "ioc" => Ok(TimeInForce::ImmediateOrCancel),
            "day-plus" => Ok(TimeInForce::DayPlus),
            "at-open" => Ok(TimeInForce::AtOpen),
            "at-close" => Ok(TimeInForce::AtClose),
            other => Err(Error::new(
                ErrorType::ParseError,
                &format!("Invalid TimeInForce: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SymbolFormat {
    Osi,
    #[default]
    Cms,
}

impl FromStr for SymbolFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "osi" => Ok(SymbolFormat::Osi),
            "cms" => Ok(SymbolFormat::Cms),
            other => Err(Error::new(
                ErrorType::ParseError,
                &format!("Invalid SymbolFormat: {}", other),
            )),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
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
    pub price: Option<String>,
    pub stop_price: Option<String>,
    pub time_in_force: TimeInForce,
    pub average_price: f64, // funny this is the only one that is a float
    pub filled_quantity: String,
    pub order_update_reason: String,
    pub text: String,
    pub strategy: Strategy,
    pub running_position: String,
}
