mod payloads;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use crate::authentication::token_manager::TokenManager;
use crate::error::{Error, ErrorType};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketClient {
    pub api_url: String,
    token_manager: TokenManager,
}

impl WebsocketClient {
    pub fn new(client_id: String, client_secret: String, api_url: String) -> Self {
        let token_manager = TokenManager::new(client_id, client_secret, api_url.clone(), api_url.clone());

        Self {
            api_url,
            token_manager
        }
    }

    pub async fn connect(&self) -> Result<(Sender<WebSocketMessage>, Receiver<WebSocketMessage>), Error> {
        tracing::debug!("Creating websocket session");

        loop {
            let (ws_stream, _) = connect_async(self.api_url.clone()).await?;

            let (mut ws_write, mut ws_read) = ws_stream.split();

            // Channels to communicate with websocket tasks
            let (tx_outbound, mut rx_outbound) = mpsc::channel::<WebSocketMessage>(100);
            let (tx_inbound, rx_inbound) = mpsc::channel::<Message>(100);

            // Task to send messages from outbound channel to websocket
            tokio::spawn(async move {
                while let Some(msg) = rx_outbound.recv().await {
                    if let Err(e) = ws_write.send(msg).await {
                        eprintln!("Websocket send error: {:?}", e);
                        break;
                    }
                }
            });

            tokio::spawn(async move {
                while let Some(msg) = ws_read.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            tracing::debug!("WebSocket Receive Text Message: {:?}", text);

                        }
                        Ok(Message::Ping(ping)) => {
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
                        _ => {}
                    }
                }
            });

            (tx_outbound, rx_inbound)
        }
    }
}
