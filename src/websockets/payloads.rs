use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Utf8Bytes;
use crate::error::{Error, ErrorType};
use crate::orders::Order;
use crate::positions::Position;
use crate::trades::Trade;

#[derive(Debug, Clone, Deserialize)]
struct RawMessage {
    pub payload: RawPayload,
}

#[derive(Debug, Clone, Deserialize)]
struct RawPayload {
    #[serde(rename = "type")]
    payload_type: PayloadType,
}

pub fn parse_message(message: Utf8Bytes) -> Result<ActivityMessage, Error> {
    // First parse the message partially to see what type it is.
    let raw_message: RawMessage = serde_json::from_str(&message)?;
    let parsed_payload_type: PayloadType = raw_message.payload.payload_type;

    // Once the type is known, reparse the full into the right variant.
    let activity_message = match parsed_payload_type {
        PayloadType::SubscribeActivityAck => {
            let parsed_message: SubscribeActivityAck = serde_json::from_str(message.as_str())?;
            ActivityMessage::SubscribeActivityAck(parsed_message)
        }
        PayloadType::ReplayComplete => {
            let parsed_message: ReplayComplete = serde_json::from_str(message.as_str())?;
            ActivityMessage::ReplayComplete(parsed_message)
        }
        PayloadType::OrderUpdate => {
            let parsed_message: OrderUpdate = serde_json::from_str(message.as_str())?;
            ActivityMessage::OrderUpdate(parsed_message)
        }
        PayloadType::TradeNotice => {
            let parsed_message: TradeNotice = serde_json::from_str(message.as_str())?;
            ActivityMessage::TradeNotice(parsed_message)
        }
        PayloadType::PositionUpdate => {
            let parsed_message: PositionUpdate = serde_json::from_str(message.as_str())?;
            ActivityMessage::PositionUpdate(parsed_message)
        }
        PayloadType::BuyingPowerUpdate => {
            let parsed_message: BuyingPowerUpdate = serde_json::from_str(message.as_str())?;
            ActivityMessage::BuyingPowerUpdate(parsed_message)
        }
        PayloadType::LocateInventoryUpdate => {
            let parsed_message: LocateInventoryUpdate = serde_json::from_str(message.as_str())?;
            ActivityMessage::LocateInventoryUpdate(parsed_message)
        }
        PayloadType::ErrorNotice => {
            let parsed_message: ErrorNotice = serde_json::from_str(message.as_str())?;
            ActivityMessage::ErrorNotice(parsed_message)
        }
        PayloadType::Heartbeat => {
            let parsed_message: Heartbeat = serde_json::from_str(message.as_str())?;
            ActivityMessage::Heartbeat(parsed_message)
        }
        _ => {
            tracing::warn!("Unknown message type received: {:?}", parsed_payload_type);
            return Err(Error::new(ErrorType::ParseError, "Unknown message type"));
        }
    };

    Ok(activity_message)
}


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
pub struct Heartbeat {
    pub timestamp: i64,
    pub payload: HeartbeatPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeartbeatPayload {
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
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

// Subscribe an Ack message format (INCOMING)
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
    // pub data: BuyingPowerUpdateData
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
