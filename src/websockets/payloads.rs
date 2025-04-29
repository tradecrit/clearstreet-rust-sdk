use serde::{Deserialize, Serialize};
use crate::orders::Order;

// Subscribe activity payload and message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeActivity {
    pub authorization: String,
    pub payload: SubscribeActivityPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeActivityPayload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub account_id: String,
}

// Ack from server
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SubscribeActivityAck {
    pub timestamp: i64,
    pub payload: SubscribeActivityAckPayload
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeActivityAckPayload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub account_id: String,
    pub success: bool,
    pub details: String,
}

//  Replay complete after connection and catch up
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayComplete {
    pub timestamp: i64,
    pub payload: ReplayCompletePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayCompletePayload {
    #[serde(rename = "type")]
    pub payload_type: String,
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
    pub payload_type: String,
    pub data: Order,
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct TradeNotice {
//     timestamp: i64,
//     sequence: i64,
//     payload: TradeNoticePayload,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct TradeNoticePayload {
//     #[serde(rename = "type")]
//     pub payload_type: String,
//     pub data: TradeNoticeData,
// }
