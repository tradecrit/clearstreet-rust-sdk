use serde::{Deserialize, Serialize};
use crate::orders::Order;
use crate::positions::Position;
use crate::trades::Trade;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PayloadType {
    #[serde(rename = "subscribe-activity")]
    SubscribeActivity,
    #[serde(rename = "subscribe-activity-ack")]
    SubscribeActivityAck,
    #[serde(rename = "replay-complete")]
    ReplayComplete,
    #[serde(rename = "order-update")]
    OrderUpdate,
    #[serde(rename = "trade-notice")]
    TradeNotice,
    #[serde(rename = "position-update")]
    PositionUpdate,
    #[serde(rename = "buying-power-update")]
    BuyingPowerUpdate,
    #[serde(rename = "locate-inventory-update")]
    LocateInventoryUpdate,
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "error-notice")]
    ErrorNotice,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Heartbeat {
    pub timestamp: i64,
    pub payload: HeartbeatPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeartbeatPayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
}

// All message formats and types, along with their serialization
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityMessage {
    SubscribeActivityAck(SubscribeActivityAck),
    ReplayComplete(ReplayComplete),
    OrderUpdate(OrderUpdate),
    TradeNotice(TradeNotice),
    PositionUpdate(PositionUpdate),
    BuyingPowerUpdate(BuyingPowerUpdate),
    LocateInventoryUpdate(LocateInventoryUpdate),
    ErrorNotice(ErrorNotice),
    Heartbeat(Heartbeat)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorNotice {
    pub timestamp: i64,
    pub payload: ErrorNoticePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorNoticePayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    pub details: String,
}

// All allowed outgoing message formats and types, along with their serialization
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeActivity {
    pub authorization: String,
    pub payload: SubscribeActivityPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeActivityPayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    pub account_id: String,
}

// Subscribe Ack message format (INCOMING)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeActivityAck {
    pub timestamp: i64,
    pub payload: SubscribeActivityAckPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeActivityAckPayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    pub success: bool,
    pub details: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayComplete {
    pub timestamp: i64,
    pub payload: ReplayCompletePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayCompletePayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderUpdate {
    pub timestamp: i64,
    pub sequence: i64,
    pub payload: OrderUpdatePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderUpdatePayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    pub data: Order,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeNotice {
    pub timestamp: i64,
    pub sequence: i64,
    pub payload: TradeNoticePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeNoticePayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    pub data: Trade,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub timestamp: i64,
    pub sequence: i64,
    pub payload: PositionUpdatePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionUpdatePayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    pub data: Position,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuyingPowerUpdate {
    pub timestamp: i64,
    pub sequence: i64,
    pub payload: BuyingPowerUpdatePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuyingPowerUpdatePayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    // pub data: BuyingPowerUpdateData, TODO add this
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocateInventoryUpdate {
    pub timestamp: i64,
    pub sequence: i64,
    pub payload: LocateInventoryUpdatePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocateInventoryUpdatePayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    // pub data: LocateInventoryUpdateData, TODO add this
}
