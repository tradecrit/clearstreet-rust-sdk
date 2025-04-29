mod payloads;

use crate::Client;
use crate::error::{Error, ErrorType};
use crate::websockets::payloads::{ActivityMessage, BuyingPowerUpdate, LocateInventoryUpdate, OrderUpdate, PayloadType, PositionUpdate, ReplayComplete, SubscribeActivity, SubscribeActivityAck, SubscribeActivityPayload, TradeNotice};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

#[derive(Debug, Clone, Deserialize)]
struct RawMessage {
    payload: RawPayload,
}

#[derive(Debug, Clone, Deserialize)]
struct RawPayload {
    #[serde(rename = "type")]
    payload_type: PayloadType,
}

fn parse_message(message: Utf8Bytes) -> Result<ActivityMessage, Error> {
    // First parse the message partially to see what type it is.
    let raw_message: RawMessage = serde_json::from_str(&message)?;
    let parsed_payload_type = raw_message.payload.payload_type;

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
    pub async fn connect_websocket(self: Arc<Self>) -> Result<Receiver<ActivityMessage>, Error> {
        tracing::debug!("Creating websocket session");

        let (tx, rx_outbound) = mpsc::channel::<ActivityMessage>(100);

        // Tricky, self has to be static as it disconnects and runs the stream as a background task.
        let this = Arc::clone(&self);
        let tx = tx.clone();
        tokio::spawn(async move {
            let _ = this.handle_stream(tx).await;
        });

        tracing::debug!("Websocket channels created");

        Ok(rx_outbound)
    }

    #[tracing::instrument(skip(self, tx))]
    async fn handle_stream(
        self: Arc<Self>,
        tx: mpsc::Sender<ActivityMessage>,
    ) -> Result<(), Error> {
        loop {
            tracing::debug!("Handling websocket connection establish");

            let bearer_token: String = self.token_manager.get_token().await?;

            let (ws_stream, _) = connect_async(self.client_options.websocket_url.clone()).await?;

            // Split to be able to get seperate read/write streams
            let (mut ws_write, mut ws_read) = ws_stream.split();

            // Task to send messages from outbound channel to websocket
            // We initialize once with the auth message and that's it.
            let authed_subscribe = SubscribeActivity {
                authorization: bearer_token,
                payload: SubscribeActivityPayload {
                    payload_type: PayloadType::SubscribeActivity,
                    account_id: self.client_options.client_id.clone(),
                },
            };

            let serialized_session = serde_json::to_string(&authed_subscribe).map_err(|e| {
                tracing::error!("Failed to serialize authentication session: {:?}", e);
                Error::new(ErrorType::SerializationError, e.to_string())
            })?;

            if let Err(e) = ws_write.send(Message::from(serialized_session)).await {
                tracing::error!("Websocket send error: {:?}", e);
                // prevent tight loop on error
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }

            let tx_outbound = tx.clone();

            while let Some(msg) = ws_read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        tracing::debug!("WebSocket Receive Text Message: {:?}", text);

                        let parsed_message: ActivityMessage = match parse_message(text) {
                            Ok(msg) => msg,
                            Err(e) => {
                                tracing::error!("Failed to parse WebSocket message: {:?}", e);
                                break;
                            }
                        };

                        // Send the parsed message to the inbound channel
                        // We relay and handle the logic, up to you to decide what to do with it.
                        if let Err(e) = tx_outbound.send(parsed_message).await {
                            tracing::error!("Failed to send message to inbound channel: {:?}", e);
                        }
                    }
                    Ok(Message::Ping(_)) => {
                        tracing::debug!("WebSocket ping received");
                        // optionally: respond with pong
                    }
                    Ok(Message::Pong(_)) => {
                        tracing::debug!("WebSocket pong received");
                    }
                    Ok(Message::Close(frame)) => {
                        tracing::warn!("WebSocket close: {:?}", frame);
                        break;
                    }
                    Ok(Message::Binary(_)) => {
                        tracing::warn!("Unexpected binary message received");
                    }
                    Err(e) => {
                        tracing::error!("WebSocket error: {:?}", e);
                        break;
                    }
                    _ => {
                        tracing::warn!("Unexpected message type received");
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    use futures_util::{SinkExt, StreamExt};
    use tokio::net::TcpListener;
    use tokio::sync::Notify;
    use tokio_tungstenite::accept_async;
    use tokio_tungstenite::tungstenite::Utf8Bytes;
    use tracing_subscriber::fmt::format::FmtSpan;

    fn setup_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::new("DEBUG"))
            .with_target(true)
            .with_level(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_line_number(true)
            .with_ansi(true)
            .with_writer(std::io::stdout)
            .try_init();
    }

    async fn create_mock_websocket_server(port: u16, ready: Arc<Notify>, shutdown: Arc<Notify>) {
        let test_url = format!("127.0.0.1:{}", port);

        let listener = TcpListener::bind(test_url).await.unwrap();

        ready.notify_waiters(); // let the test know we're readylistening

        let (stream, _) = listener.accept().await.unwrap();
        let mut ws_stream = accept_async(stream).await.unwrap();

        // Wait for the auth message
        let auth_msg = ws_stream.next().await.unwrap().unwrap();
        assert!(matches!(auth_msg, Message::Text(_)));

        // Send a mocked message to the client
        let fake_order_update = r#"
            {
              "timestamp": 0,
              "sequence": 0,
              "payload": {
                "type": "order-update",
                "data": {
                  "created_at": 0,
                  "updated_at": 0,
                  "order_id": "12390213",
                  "reference_id": "test-ref",
                  "version": 1,
                  "account_id": "test-account",
                  "account_number": "ACC001",
                  "state": "open",
                  "status": "new",
                  "symbol": "AAPL",
                  "order_type": "limit",
                  "side": "buy",
                  "quantity": "100",
                  "price": "123.45",
                  "stop_price": "123.45",
                  "time_in_force": "day",
                  "average_price": 0,
                  "filled_quantity": "0",
                  "order_update_reason": "place",
                  "text": "test",
                  "strategy": null,
                  "running_position": "0"
                }
              }
            }
        "#;

        ws_stream.send(Message::Text(Utf8Bytes::from(fake_order_update.to_string()))).await.unwrap();

        // Wait for shutdown
        shutdown.notified().await;
    }

    #[tokio::test]
    async fn test_client_receives_order_update() {
        setup_tracing();

        let port = 12345;
        let test_url = format!("ws://127.0.0.1:{}", port);

        let ready = Arc::new(tokio::sync::Notify::new());
        let shutdown = Arc::new(tokio::sync::Notify::new());

        // Launch mock server
        let mock_ready = ready.clone();
        let mock_shutdown = shutdown.clone();
        tokio::spawn(async move {
            create_mock_websocket_server(port, mock_ready, mock_shutdown).await;
        });

        // Wait for server to bind
        ready.notified().await;

        let temp_client = Client::new_with_token("".to_string(), test_url.clone(), "mock-token".to_string());

        let client = Arc::new(temp_client);
        let mut rx = client.connect_websocket().await.unwrap();

        // Receive and assert message
        let msg = tokio::time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("Did not receive message in time")
            .unwrap_or_else(|| panic!("WebSocket receiver dropped before receiving a message"));

        match msg {
            ActivityMessage::OrderUpdate(order_update) => {
                let recv_symbol = order_update.payload.data.symbol;
                assert_eq!(recv_symbol, "AAPL");
            }
            _ => panic!("Unexpected message variant"),
        }

        // Shut down mock server
        shutdown.notify_waiters();
    }
}
