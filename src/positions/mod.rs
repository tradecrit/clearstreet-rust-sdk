use crate::error::ErrorType::HttpError;
use crate::utils::parse_response;
use crate::Client;
use crate::Error;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub account_id: String,
    pub account_number: String,
    pub symbol: String,
    pub quantity: String,
    pub average_cost: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetPositionParams {
    pub account_id: String,
    pub symbol: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListPositionsResponse {
    pub data: Vec<Position>,
    pub next_page_token: Option<String>,
}

impl Client {
    pub async fn get_position(&self, token: &str, params: GetPositionParams) -> Result<Position, Error> {
        let client = self.build_authenticated_client(token).await?;

        let url = format!("{}/studio/v2/accounts/{}/positions/{}",  self.client_options.api_url, params.account_id, params.symbol);

        let request_builder = client.get(&url);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: Position = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
    }

    pub async fn list_positions(&self, token: &str, account_id: &str) -> Result<ListPositionsResponse, Error> {
        tracing::debug!("list_positions: {:?}", account_id);

        let client = self.build_authenticated_client(token).await?;

        let url = format!("{}/studio/v2/accounts/{}/positions",  self.client_options.api_url, account_id);

        let request_builder = client.get(&url);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: ListPositionsResponse = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
    }
}

