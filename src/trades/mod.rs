use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use crate::{Client};
use crate::error::{Error};
use crate::error::ErrorType::HttpError;
use crate::utils::parse_response;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub created_at: i64,
    pub account_id: String,
    pub account_number: String,
    pub trade_id: String,
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub quantity: String,
    pub price: String,
    pub running_position: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTradeRequest {
    pub account_id: String,
    pub trade_id: String,
}

impl Client {
    pub async fn get_trade(&self, token: &str, params: GetTradeRequest) -> Result<Trade, Error> {
        let url = format!("{}/studio/v2/accounts/{}/trades/{}", self.client_options.api_url, params.account_id, params.trade_id);

        let client = self.build_authenticated_client(token).await?;

        let request_builder: RequestBuilder = client.get(&url);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: Trade = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
    }
}
