use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::{utils, Client};
use crate::error::{Error};
use crate::error::ErrorType::HttpError;
use crate::orders::SymbolFormat;
use crate::utils::parse_response;

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolDetail {
    pub symbol: String,
    pub symbol_format: SymbolFormat,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Instrument {
    pub symbols: Vec<SymbolDetail>,
    pub asset_class: String,
    pub primary_exchange: String,
    pub description: String,
}

impl Client {
    #[tracing::instrument(skip(self, token))]
    pub async fn get_instrument(&self, token: &str, symbol: &str) -> Result<Instrument, Error> {
        let client = self.build_authenticated_client(token).await?;

        let url = format!("{}/studio/v2/instruments/{}",  self.client_options.api_url, symbol);

        let request_builder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: Instrument = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
    }
}
