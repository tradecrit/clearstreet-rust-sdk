mod payloads;

use crate::Client;
use crate::error::{Error, ErrorType};
pub use crate::websockets::payloads::{ActivityMessage, BuyingPowerUpdate, ErrorNotice, LocateInventoryUpdate, OrderUpdate, PayloadType, PositionUpdate, ReplayComplete, SubscribeActivity, SubscribeActivityAck, SubscribeActivityPayload, TradeNotice};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitStream;
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use crate::websockets::payloads::Heartbeat;

#[derive(Debug, Clone, Deserialize)]
struct RawMessage {
    payload: RawPayload,
}

#[derive(Debug, Clone, Deserialize)]
struct RawPayload {
    #[serde(rename = "type")]
    payload_type: PayloadType,
}

pub type WebsocketStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub fn parse_message(message: Utf8Bytes) -> Result<ActivityMessage, Error> {
    // First parse the message partially to see what type it is.
    let raw_message: RawMessage = serde_json::from_str(&message)?;
    let parsed_payload_type = raw_message.payload.payload_type;

    // Once the type is known, reparse the full into the right variant.
    let activity_message = match parsed_payload_type {
        PayloadType::SubscribeActivityAck => {
            tracing::debug!("SubscribeActivityAck");
            let parsed_message: SubscribeActivityAck = serde_json::from_str(message.as_str())?;
            ActivityMessage::SubscribeActivityAck(parsed_message)
        }
        PayloadType::ReplayComplete => {
            tracing::debug!("ReplayComplete");
            let parsed_message: ReplayComplete = serde_json::from_str(message.as_str())?;
            ActivityMessage::ReplayComplete(parsed_message)
        }
        PayloadType::OrderUpdate => {
            tracing::debug!("OrderUpdate");
            let parsed_message: OrderUpdate = serde_json::from_str(message.as_str())?;
            ActivityMessage::OrderUpdate(parsed_message)
        }
        PayloadType::TradeNotice => {
            tracing::debug!("TradeNotice");
            let parsed_message: TradeNotice = serde_json::from_str(message.as_str())?;
            ActivityMessage::TradeNotice(parsed_message)
        }
        PayloadType::PositionUpdate => {
            tracing::debug!("PositionUpdate");
            let parsed_message: PositionUpdate = serde_json::from_str(message.as_str())?;
            ActivityMessage::PositionUpdate(parsed_message)
        }
        PayloadType::BuyingPowerUpdate => {
            tracing::debug!("BuyingPowerUpdate");
            let parsed_message: BuyingPowerUpdate = serde_json::from_str(message.as_str())?;
            ActivityMessage::BuyingPowerUpdate(parsed_message)
        }
        PayloadType::LocateInventoryUpdate => {
            tracing::debug!("LocateInventoryUpdate");
            let parsed_message: LocateInventoryUpdate = serde_json::from_str(message.as_str())?;
            ActivityMessage::LocateInventoryUpdate(parsed_message)
        }
        PayloadType::ErrorNotice => {
            tracing::debug!("ErrorNotice");
            let parsed_message: ErrorNotice = serde_json::from_str(message.as_str())?;
            ActivityMessage::ErrorNotice(parsed_message)
        }
        PayloadType::Heartbeat => {
            tracing::debug!("Heartbeat");
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

impl Client {
    // Establish the session for the websocket manager. Messages are raw json.
    // On failure/disconnection, it will retry the connection.
    // Authentication is handled by the TokenManager and has to be sent to establish the connection.
    // API hard times out at 24 hours.
    pub async fn connect_websocket(&self, token: &str, account_id: &str) -> Result<WebsocketStream, Error> {
        tracing::debug!("Creating websocket session");
        let (ws_stream, _) = connect_async(self.client_options.websocket_url.clone()).await?;

        // Split to be able to get seperate read/write streams
        let (mut ws_write, ws_read) = ws_stream.split();

        // Task to send messages from outbound channel to websocket
        // We initialize once with the auth message and that's it.
        let authed_subscribe = SubscribeActivity {
            authorization: token.to_string(),
            payload: SubscribeActivityPayload {
                payload_type: PayloadType::SubscribeActivity,
                account_id: account_id.to_string(),
            },
        };

        let serialized_session = serde_json::to_string(&authed_subscribe)?;

        tracing::debug!("Sending auth message: {}", serialized_session);
        ws_write.send(Message::from(serialized_session)).await?;

        Ok(ws_read)
    }
}

