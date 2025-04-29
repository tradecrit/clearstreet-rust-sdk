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
    pub async fn get_position(&self, params: GetPositionParams) -> Result<Position, Error> {
        //      --url https://api.clearstreet.io/studio/v2/accounts/asdasd/positions/APL \
        let client = self.build_authenticated_client().await?;

        let url = format!("{}/studio/v2/accounts/{}/positions/{}",  self.client_options.api_url, params.account_id, params.symbol);

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

        let url = format!("{}/studio/v2/accounts/{}/positions",  self.client_options.api_url, account_id);

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

#[cfg(test)]
mod tests {
    use tracing_subscriber::fmt::format::FmtSpan;
    use crate::Client;
    use mockito::{Server};
    use crate::positions::GetPositionParams;

    fn setup_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
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
    async fn test_get_position() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("GET", "/studio/v2/accounts/100000/positions/AAPL")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "account_id": "100000",
                "account_number": "ACC0001",
                "symbol": "AAPL",
                "quantity": "100",
                "average_cost": 0
            }"#)
            .create_async()
            .await;

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let params = GetPositionParams {
            account_id: "100000".to_string(),
            symbol: "AAPL".to_string(),
        };

        let result = client.get_position(params).await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.symbol, "AAPL");
    }

    #[tokio::test]
    async fn test_list_positions() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("GET", "/studio/v2/accounts/100000/positions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "data": [{
                        "account_id": "100000",
                        "account_number": "ACC0001",
                        "symbol": "AAPL",
                        "quantity": "100",
                        "average_cost": 0
                    }],
                    "next_page_token": "abc123"
                }"#)
            .create_async()
            .await;

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let account_id = "100000".to_string();

        let result = client.list_positions(&account_id).await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.data[0].symbol, "AAPL");
    }
}
