use crate::client::async_client::AsyncClient;
use crate::error::Error;
use crate::utils::parse_response;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SymbolDetail {
    pub symbol: String,
    pub symbol_format: String,
}

#[derive(Serialize, Deserialize)]
pub struct Instrument {
    pub symbols: Vec<SymbolDetail>,
    pub asset_class: String,
    pub primary_exchange: String,
    pub description: String,
}

#[cfg(feature = "async")]
pub async fn get_instrument(client: &AsyncClient, symbol: &str) -> Result<Instrument, Error> {
    let api_url: &str = &client.client_options.api_url;

    let url = format!("{api_url}/studio/v2/instruments/{symbol}");

    let request_builder = client.client.get(&url);
    let response: Response = request_builder.send().await?;

    parse_response::<Instrument>(response).await
}
