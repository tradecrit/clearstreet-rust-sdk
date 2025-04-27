use crate::error::ErrorType::HttpError;
use crate::error::{BrokerApiError, Error};
use crate::{utils, Client};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::utils::{parse_response};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub account_id: i64,
    pub account_number: String,
    pub entity_id: i64,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountsResponse {
    pub data: Vec<Account>,
}

impl Client {
    pub async fn get_account(&self, account_id: &str) -> Result<Account, Error> {
        let client = self.build_authenticated_client().await?;

        let url = format!("{}/accounts/{}", self.api_url, account_id);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if !response.status().is_success() {
            let broker_error: BrokerApiError = parse_response(response).await?;
            tracing::error!("{}", broker_error);
            return Err(Error::new(HttpError, broker_error.to_string()));
        }

        let body: Account = parse_response(response).await?;

        tracing::debug!("Account info: {:?}", body);

        Ok(body)
    }

    pub async fn get_accounts(&self) -> Result<GetAccountsResponse, Error> {
        let client = self.build_authenticated_client().await?;

        let url = format!("{}/accounts", self.api_url);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if !response.status().is_success() {
            let broker_error: BrokerApiError = parse_response(response).await?;
            tracing::error!("{}", broker_error);
            return Err(Error::new(HttpError, broker_error.to_string()));
        }

        let body: GetAccountsResponse = parse_response(response).await?;

        tracing::debug!("Account info: {:?}", body);

        Ok(body)
    }
}



#[cfg(test)]
mod tests {
    use crate::Client;
    use mockito::{Server};

    #[tokio::test]
    async fn test_get_account_info_success() {
        let mut server = Server::new_async().await;

        let _mock = server
            .mock("GET", "/accounts/me")
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

        let client = Client::new_with_token(server.url(), "test-token".into());

        let result = client.get_account_info().await;
        assert!(result.is_ok());
    }
}