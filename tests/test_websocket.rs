use std::env;
use dotenvy::dotenv;
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::Message;
use tracing_subscriber::fmt::format::FmtSpan;
use clearstreet::{Client, ClientOptions};
use clearstreet::websockets::{parse_message, ActivityMessage, WebsocketStream};

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

#[tokio::test]
async fn test_get_order() {
    dotenv().ok().unwrap_or_default();
    setup_tracing();

    let options = ClientOptions {
        client_id: env::var("BROKER_CLIENT_ID").unwrap(),
        client_secret: env::var("BROKER_CLIENT_SECRET").unwrap(),
        account_id: env::var("BROKER_ACCOUNT_ID").unwrap(),
        ..Default::default()
    };

    let client = Client::init(options).await.expect("Failed to initialize client");

    let mut ws_read: WebsocketStream = client.connect_websocket().await.unwrap();

    while let Some(msg) = ws_read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let parsed = parse_message(text);

                if parsed.is_ok() {
                    let message = parsed.unwrap();

                    match message {
                        ActivityMessage::SubscribeActivityAck(ack) => {
                            tracing::info!("SubscribeActivityAck: {:?}", ack);
                        }
                        ActivityMessage::OrderUpdate(update) => {
                            tracing::info!("OrderUpdate: {:?}", update);
                        }
                        ActivityMessage::ErrorNotice(err) => {
                            tracing::error!("ErrorNotice: {:?}", err);
                        }

                        _ => {}
                    }
                }
            }
            _ => {
                tracing::warn!("Unexpected message: {:?}", msg);
            }
        }
    }

    tracing::warn!("WebSocket stream closed");
}