use crate::error::ErrorType::HttpError;
use crate::error::{Error};
use crate::{utils, Client};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::utils::{parse_response};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Account {
    pub account_id: String,
    pub account_number: String,
    pub entity_id: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GetAccountsResponse {
    pub data: Vec<Account>,
}

impl Client {
    #[tracing::instrument(skip(self, token, account_id))]
    pub async fn get_account(&self, token: &str, account_id: &str) -> Result<Account, Error> {
        let client = self.build_authenticated_client(token).await?;

        let url = format!("{}/studio/v2/accounts/{}",  self.client_options.api_url, account_id);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: Account = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
    }

    #[tracing::instrument(skip(self, token))]
    pub async fn get_accounts(&self, token: &str) -> Result<GetAccountsResponse, Error> {
        let client = self.build_authenticated_client(token).await?;

        let url = format!("{}/studio/v2/accounts",  self.client_options.api_url);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: GetAccountsResponse = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
    }
}



#[cfg(test)]
mod tests {
    use tracing_subscriber::fmt::format::FmtSpan;
use crate::{Client, ClientOptions};
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
            .mock("GET", "/studio/v2/accounts/123abc")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "account_id": "123abc",
                "account_number": "ACC456",
                "entity_id": "456abc",
                "name": "Test Account"
            }"#)
            .create_async()
            .await;

        let options = ClientOptions {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            ..Default::default()
        };

        let client = Client::new(options);
        let token = client.fetch_new_token().await.unwrap();

        let result = client.get_account(&token.access_token, "123abc").await;

        assert!(result.is_ok());

        let account = result.unwrap();
        assert_eq!(account.account_id, "123abc");
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
                    "account_id": "123abc",
                    "account_number": "ACC456",
                    "entity_id": "456abc",
                    "name": "Test Account"
                }
            ]
        }"#)
            .create_async()
            .await;

        let options = ClientOptions {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            ..Default::default()
        };

        let client = Client::new(options);
        let token = client.fetch_new_token().await.unwrap();

        let result = client.get_accounts(&token.access_token).await;
        assert!(result.is_ok());

        let accounts_response = result.unwrap();
        assert_eq!(accounts_response.data.len(), 1);
    }
}
