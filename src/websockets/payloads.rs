use serde::{Deserialize, Serialize};
use crate::orders::Order;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeRequest {
    pub authorization: String,
    pub payload: SubscribeRequestPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeRequestPayload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub account_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SubscribeAckMessage {
    pub timestamp: i64,
    pub payload: SubscribeResponsePayload
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeResponsePayload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub account_id: String,
    pub success: bool,
    pub details: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceCompleteMessage {
    pub timestamp: i64,
    pub payload: ReplayCompletePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayCompletePayload {
    #[serde(rename = "type")]
    pub payload_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderUpdateMessage {
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
