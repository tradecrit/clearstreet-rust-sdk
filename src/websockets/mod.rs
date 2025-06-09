mod payloads;
use crate::error::Error;

use crate::client::async_client::AsyncClient;
pub use crate::websockets::payloads::{
    ActivityMessage, BuyingPowerUpdate, ErrorNotice, LocateInventoryUpdate, OrderUpdate,
    PayloadType, PositionUpdate, ReplayComplete, SubscribeActivity, SubscribeActivityAck,
    SubscribeActivityPayload, TradeNotice,
};

use crate::client::sync_client::SyncClient;
use tokio_tungstenite::{
    WebSocketStream, connect_async, tungstenite::protocol::Message,
};
use tungstenite::{Utf8Bytes};

#[cfg(feature = "async")]
use futures_util::SinkExt;

#[cfg(feature = "async")]
pub async fn connect_websocket(
    client: &AsyncClient,
) -> Result<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Error> {
    tracing::debug!("Creating websocket session");

    let ws_url = client.client_options.websocket_url.clone();

    let (mut ws_stream, _) = connect_async(ws_url)
        .await?;

    // Build an auth message
    let token = &client.token;
    let account_id = &client.client_options.account_id;

    let auth_msg = SubscribeActivity {
        authorization: token.to_string(),
        payload: SubscribeActivityPayload {
            payload_type: PayloadType::SubscribeActivity,
            account_id: account_id.to_string(),
        },
    };

    let msg_json = serde_json::to_string(&auth_msg)?;
    tracing::debug!("Sending auth message: {}", msg_json);

    ws_stream
        .send(Message::Text(Utf8Bytes::from(msg_json)))
        .await?;

    Ok(ws_stream)
}

#[cfg(feature = "sync")]
pub fn connect_websocket_blocking(
    client: &SyncClient,
) -> Result<tungstenite::protocol::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, Error> {
    tracing::debug!("Creating blocking websocket session");

    let (mut ws_stream, _response) = tungstenite::connect(&client.client_options.websocket_url)?;

    let token = &client.token;
    let account_id = &client.client_options.account_id;

    let auth_msg = SubscribeActivity {
        authorization: token.to_string(),
        payload: SubscribeActivityPayload {
            payload_type: PayloadType::SubscribeActivity,
            account_id: account_id.to_string(),
        },
    };

    let msg_json = serde_json::to_string(&auth_msg)?;
    tracing::debug!("Sending auth message: {}", msg_json);

    ws_stream.send(Message::Text(Utf8Bytes::from(msg_json)))?;

    Ok(ws_stream)
}
