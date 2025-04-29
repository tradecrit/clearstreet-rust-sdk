mod payloads;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketClient {
    pub url: String,
    pub token: String,
}


impl WebsocketClient {

}