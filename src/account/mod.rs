use crate::error::ErrorType::HttpError;
use crate::error::{Error};
use crate::{Client};
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
    #[tracing::instrument(skip(self, token, account_id))]
    pub async fn get_account(&self, token: &str, account_id: &str) -> Result<Account, Error> {
        let client = self.build_authenticated_client(token).await?;

        let url = format!("{}/studio/v2/accounts/{}",  self.client_options.api_url, account_id);

        let request_builder = client.get(&url);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

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

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

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
