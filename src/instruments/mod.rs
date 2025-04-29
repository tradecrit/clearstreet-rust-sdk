use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::{utils, Client};
use crate::error::{BrokerApiError, Error};
use crate::error::ErrorType::HttpError;
use crate::utils::parse_response;

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolDetail {
    pub symbol: String,
    pub symbol_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    pub symbols: Vec<SymbolDetail>,
    pub asset_class: String,
    pub primary_exchange: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SymbolFormat {
    OSI,
    CMS,
}

impl Client {
    #[tracing::instrument(skip(self))]
    pub async fn get_instrument(&self, symbol: &str, symbol_format: SymbolFormat) -> Result<Instrument, Error> {
        let client = self.build_authenticated_client().await?;

        let url = format!("{}/studio/v2/instruments/{}", self.api_url, symbol);

        let request_builder = client.get(&url)
            .query(&[("symbol_format", symbol_format)]);

        let response: Response = utils::request(request_builder).await?;

        if response.status().is_success() {
            let body: Instrument = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
        }

        let broker_error: BrokerApiError = parse_response(response).await?;
        tracing::error!("{}", broker_error);
        Err(Error::new(HttpError, broker_error.to_string()))
    }
}