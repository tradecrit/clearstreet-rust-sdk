use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use crate::{utils, Client};
use crate::error::{BrokerApiError, Error};
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
    pub async fn get_trade(&self,  get_trade_request: GetTradeRequest) -> Result<Trade, Error> {
        let url = format!("{}/studio/v2/accounts/{}/trades/{}", self.client_options.api_url, get_trade_request.account_id, get_trade_request.trade_id);

        let client = self.build_authenticated_client().await?;

        let request_builder: RequestBuilder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: Trade = parse_response(response).await?;
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
    use crate::Client;
    use mockito::Server;
    use tracing_subscriber::fmt::format::FmtSpan;
    use crate::trades::GetTradeRequest;

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
    async fn test_get_trade() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("GET", "/studio/v2/accounts/100000/trades/12390213")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "created_at": 0,
                "account_id": "100000",
                "account_number": "ACC0001",
                "trade_id": "12390213",
                "order_id": "12390213",
                "symbol": "AAPL",
                "side": "buy",
                "quantity": "100",
                "price": "123.99",
                "running_position": "100"
            }"#)
            .create_async()
            .await;

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let request = GetTradeRequest {
            account_id: "100000".to_string(),
            trade_id: "12390213".to_string(),
        };

        let result = client.get_trade(request).await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.symbol, "AAPL");
    }
}
