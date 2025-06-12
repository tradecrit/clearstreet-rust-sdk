use clearstreet::client::AsyncClearstreetClient;
use clearstreet::client::async_client::AsyncClient;
use futures_util::StreamExt;
use std::env;
use tungstenite::{Message};

#[tokio::test]
pub async fn test_websocket() {
    dotenvy::dotenv().ok();

    let client_options = clearstreet::client::ClientOptions {
        client_id: env::var("CLEARSTREET_CLIENT_ID").expect("CLEARSTREET_CLIENT_ID not set"),
        client_secret: env::var("CLEARSTREET_CLIENT_SECRET")
            .expect("CLEARSTREET_CLIENT_SECRET not set"),
        account_id: env::var("CLEARSTREET_ACCOUNT_ID").expect("CLEARSTREET_ACCOUNT_ID not set"),
        ..Default::default()
    };

    let client = AsyncClient::create(client_options).await;

    #[cfg(feature = "async")]
    {
        let get = client.connect_websocket().await;
        assert!(get.is_ok());

        let mut order = get.unwrap();

        while let Some(msg) = order.next().await {
            let message = msg.unwrap();

            match message {
                Message::Text(text) => {
                    let parsed_message = match clearstreet::websockets::payloads::parse_message(text) {
                        Ok(parsed) => parsed,
                        Err(e) => {
                            eprintln!("Failed to parse message: {:?}", e);
                            break;
                        }
                    };

                    println!("Received message: {:?}", parsed_message);
                }
                _ => {
                    continue;
                }
            }
        }
    }
}
