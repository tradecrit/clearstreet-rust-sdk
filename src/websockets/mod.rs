mod payloads;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketClient {
    pub url: String,
    pub token: String,
}

// impl WebsocketClient {
//     /// Connects to the websocket and returns (tx, rx) channels for communication
//     pub async fn connect(&self) -> Result<(Sender<Message>, Receiver<Message>), Error> {
//         let full_url = format!("{}?token={}", self.url, self.token);
//
//         let (ws_stream, _) = connect_async(&full_url)
//             .await?;
//
//         let (mut ws_write, mut ws_read) = ws_stream.split();
//
//         // Channels to communicate with websocket tasks
//         let (tx_outbound, mut rx_outbound) = mpsc::channel::<Message>(100);
//         let (tx_inbound, rx_inbound) = mpsc::channel::<Message>(100);
//
//         // Task to send messages from outbound channel to websocket
//         tokio::spawn(async move {
//             while let Some(msg) = rx_outbound.recv().await {
//                 if let Err(e) = ws_write.send(msg).await {
//                     eprintln!("Websocket send error: {:?}", e);
//                     break;
//                 }
//             }
//         });
//
//         // Task to receive messages from websocket and forward to inbound channel
//         tokio::spawn(async move {
//             while let Some(msg) = ws_read.next().await {
//                 match msg {
//                     Ok(message) => {
//                         if tx_inbound.send(message).await.is_err() {
//                             eprintln!("Inbound receiver dropped");
//                             break;
//                         }
//                     }
//                     Err(e) => {
//                         eprintln!("Websocket receive error: {:?}", e);
//                         break;
//                     }
//                 }
//             }
//         });
//
//         (tx_outbound, rx_inbound)
//     }
// }
