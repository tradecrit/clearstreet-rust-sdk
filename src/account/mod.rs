use crate::error::ErrorType::HttpError;
use crate::error::{BrokerApiError, Error};
use crate::{utils, Client};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::utils::{parse_response};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub account_id: String,
    pub account_number: String,
    pub entity_id: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountsResponse {
    pub data: Vec<Account>,
}

impl Client {
    #[tracing::instrument(skip(self))]
    pub async fn get_account(&self, account_id: &str) -> Result<Account, Error> {
        let client = self.build_authenticated_client().await?;

        let url = format!("{}/studio/v2/accounts/{}",  self.client_options.api_url, account_id);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: Account = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_accounts(&self) -> Result<GetAccountsResponse, Error> {
        let client = self.build_authenticated_client().await?;

        let url = format!("{}/studio/v2/accounts",  self.client_options.api_url);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: GetAccountsResponse = parse_response(response).await?;
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
    async fn test_get_account() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("GET", "/studio/v2/accounts/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "account_id": 123,
                "account_number": "ACC456",
                "entity_id": 456,
                "name": "Test Account"
            }"#)
            .create_async()
            .await;

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let result = client.get_account("123").await;

        assert!(result.is_ok());

        let account = result.unwrap();
        assert_eq!(account.account_id, "123");
    }

    #[tokio::test]
    async fn test_get_accounts() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("GET", "/studio/v2/accounts")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
            "data": [
                {
                    "account_id": 123,
                    "account_number": "ACC456",
                    "entity_id": 456,
                    "name": "Test Account"
                }
            ]
        }"#)
            .create_async()
            .await;

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let result = client.get_accounts().await;
        assert!(result.is_ok());

        let accounts_response = result.unwrap();
        assert_eq!(accounts_response.data.len(), 1);
    }
}
