use crate::error::ErrorType::HttpError;
use crate::error::BrokerApiError;
use crate::utils::parse_response;
use crate::utils;
use reqwest::Response;
use crate::Error;
use serde::{Deserialize, Serialize};
use crate::Client;

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
    pub async fn get_positions(&self, get_position_params: GetPositionParams) -> Result<Position, Error> {
        //      --url https://api.clearstreet.io/studio/v2/accounts/asdasd/positions/APL \
        let client = self.build_authenticated_client().await?;

        let url = format!("{}/studio/v2/accounts/{}/positions/{}", self.api_url, get_position_params.account_id, get_position_params.symbol);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: Position = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }

    pub async fn list_positions(&self, account_id: &str) -> Result<ListPositionsResponse, Error> {
        tracing::debug!("list_positions: {:?}", account_id);

        let client = self.build_authenticated_client().await?;

        let url = format!("{}/studio/v2/accounts/{}/positions", self.api_url, account_id);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: ListPositionsResponse = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }
}