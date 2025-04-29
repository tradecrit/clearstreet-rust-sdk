use serde::{Deserialize, Serialize};

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
struct SubscribeResponse {
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
pub struct ReplaceCompleteResponse{
    pub timestamp: i64,
    pub payload: ReplaceCompletePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceCompletePayload {
    #[serde(rename = "type")]
    pub payload_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketClient {
    pub url: String,
    pub token: String,
}

impl WebsocketClient {

}