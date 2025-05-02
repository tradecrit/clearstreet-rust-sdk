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
            return Err(Error::new(ErrorType::ParseError, "Unknown message type".to_string()));
        }
    };

    Ok(activity_message)
}

impl Client {
    // Establish the session for the websocket manager. Messages are raw json.
    // On failure/disconnection, it will retry the connection.
    // Authentication is handled by the TokenManager and has to be sent to establish the connection.
    // API hard times out at 24 hours.
    pub async fn connect_websocket(&self) -> Result<WebsocketStream, Error> {
        tracing::debug!("Creating websocket session");

        tracing::debug!("Handling websocket connection establish");

        let bearer_token: String = self.token_manager.get_token().await?;

        let (ws_stream, _) = connect_async(self.client_options.websocket_url.clone()).await?;

        // Split to be able to get seperate read/write streams
        let (mut ws_write, ws_read) = ws_stream.split();

        // Task to send messages from outbound channel to websocket
        // We initialize once with the auth message and that's it.
        let authed_subscribe = SubscribeActivity {
            authorization: bearer_token,
            payload: SubscribeActivityPayload {
                payload_type: PayloadType::SubscribeActivity,
                account_id: self.client_options.account_id.clone(),
            },
        };

        let serialized_session = serde_json::to_string(&authed_subscribe).map_err(|e| {
            tracing::error!("Failed to serialize authentication session: {:?}", e);
            Error::new(ErrorType::SerializationError, e.to_string())
        })?;

        tracing::debug!("Sending subscribe to websocket: {:?}", serialized_session);

        if let Err(e) = ws_write.send(Message::from(serialized_session)).await {
            tracing::error!("Websocket send error: {:?}", e);
            return Err(Error::new(ErrorType::SerializationError, "Websocket send error".to_string()));
        }

        Ok(ws_read)
    }
}

#[cfg(test)]
mod tests {
    // use std::sync::Arc;
    // use std::time::Duration;
    // use super::*;
    //
    // use futures_util::{SinkExt, StreamExt};
    // use tokio::net::TcpListener;
    // use tokio::sync::Notify;
    // use tokio_tungstenite::accept_async;
    // use tokio_tungstenite::tungstenite::Utf8Bytes;
    // use tracing_subscriber::fmt::format::FmtSpan;
    //
    // fn setup_tracing() {
    //     let _ = tracing_subscriber::fmt()
    //         .with_env_filter(tracing_subscriber::EnvFilter::new("DEBUG"))
    //         .with_target(true)
    //         .with_level(true)
    //         .with_thread_ids(true)
    //         .with_thread_names(true)
    //         .with_span_events(FmtSpan::CLOSE)
    //         .with_line_number(true)
    //         .with_ansi(true)
    //         .with_writer(std::io::stdout)
    //         .try_init();
    // }
    //
    // async fn create_mock_websocket_server(port: u16, ready: Arc<Notify>, shutdown: Arc<Notify>) {
    //     let test_url = format!("127.0.0.1:{}", port);
    //
    //     let listener = TcpListener::bind(test_url).await.unwrap();
    //
    //     ready.notify_waiters(); // let the test know we're readylistening
    //
    //     let (stream, _) = listener.accept().await.unwrap();
    //     let mut ws_stream = accept_async(stream).await.unwrap();
    //
    //     // Wait for the auth message
    //     let auth_msg = ws_stream.next().await.unwrap().unwrap();
    //     assert!(matches!(auth_msg, Message::Text(_)));
    //
    //     // Send a mocked message to the client
    //     let fake_order_update = r#"
    //         {
    //           "timestamp": 0,
    //           "sequence": 0,
    //           "payload": {
    //             "type": "order-update",
    //             "data": {
    //               "created_at": 0,
    //               "updated_at": 0,
    //               "order_id": "12390213",
    //               "reference_id": "test-ref",
    //               "version": 1,
    //               "account_id": "test-account",
    //               "account_number": "ACC001",
    //               "state": "open",
    //               "status": "new",
    //               "symbol": "AAPL",
    //               "order_type": "limit",
    //               "side": "buy",
    //               "quantity": "100",
    //               "price": "123.45",
    //               "stop_price": "123.45",
    //               "time_in_force": "day",
    //               "average_price": 0,
    //               "filled_quantity": "0",
    //               "order_update_reason": "place",
    //               "text": "test",
    //               "strategy": null,
    //               "running_position": "0"
    //             }
    //           }
    //         }
    //     "#;
    //
    //     ws_stream.send(Message::Text(Utf8Bytes::from(fake_order_update.to_string()))).await.unwrap();
    //
    //     // Wait for shutdown
    //     shutdown.notified().await;
    // }
    //
    // #[tokio::test]
    // async fn test_client_receives_order_update() {
    //     setup_tracing();
    //
    //     let port = 12345;
    //     let test_url = format!("ws://127.0.0.1:{}", port);
    //
    //     let ready = Arc::new(tokio::sync::Notify::new());
    //     let shutdown = Arc::new(tokio::sync::Notify::new());
    //
    //     // Launch mock server
    //     let mock_ready = ready.clone();
    //     let mock_shutdown = shutdown.clone();
    //     tokio::spawn(async move {
    //         create_mock_websocket_server(port, mock_ready, mock_shutdown).await;
    //     });
    //
    //     // Wait for server to bind
    //     ready.notified().await;
    //
    //     match msg {
    //         ActivityMessage::OrderUpdate(order_update) => {
    //             let recv_symbol = order_update.payload.data.symbol;
    //             assert_eq!(recv_symbol, "AAPL");
    //         }
    //         _ => panic!("Unexpected message variant"),
    //     }
    //
    //     // Shut down mock server
    //     shutdown.notify_waiters();
    // }
}
