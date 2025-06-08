mod payloads;

use crate::error::{Error, ErrorType};
pub use crate::websockets::payloads::{ActivityMessage, BuyingPowerUpdate, ErrorNotice, LocateInventoryUpdate, OrderUpdate, PayloadType, PositionUpdate, ReplayComplete, SubscribeActivity, SubscribeActivityAck, SubscribeActivityPayload, TradeNotice};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitStream;
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{connect, Message, Utf8Bytes};
use crate::client::async_client::AsyncClient;
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

#[cfg(feature = "async")]
pub async fn connect_websocket(client: AsyncClient, token: &str, account_id: &str) -> Result<WebsocketStream, Error> {
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


#[cfg(feature = "sync")]
pub fn connect_websocket_blocking(client: AsyncClient, token: &str, account_id: &str) -> Result<WebsocketStream, Error> {
    tracing::debug!("Creating websocket session");
    let (ws_stream, _) = connect(client.client_options.websocket_url.clone())?;

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
    ws_write.send(Message::from(serialized_session))?;

    Ok(ws_read)
}
