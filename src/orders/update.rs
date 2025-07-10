use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::error::ErrorType::HttpError;

#[cfg(feature="async")]
use crate::client::async_client::AsyncClient;
#[cfg(feature="sync")]
use crate::client::sync_client::SyncClient;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequestBody {
    pub quantity: String,
    pub price: Option<String>,
    pub stop_price: Option<String>,
}


#[cfg(feature = "async")]
pub async fn update_order(
    client: &AsyncClient,
    order_id: &str,
    body: UpdateOrderRequestBody,
) -> Result<(), Error> {
    let api_url: &str = &client.client_options.api_url;
    let account_id: &str = &client.client_options.account_id;

    let url: String = format!("{api_url}/studio/v2/accounts/{account_id}/orders/{order_id}");

    let request_builder: RequestBuilder = client.client.patch(&url).json(&body);

    let response: Response = request_builder
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await?;
        return Err(Error::new(HttpError, &format!("Error: {} - {}", status, error_body)));
    }

    Ok(())
}

#[cfg(feature = "sync")]
pub fn update_order_blocking(
    client: &SyncClient,
    order_id: &str,
    body: UpdateOrderRequestBody,
) -> Result<(), Error> {
    let url: String = format!(
        "{}/studio/v2/accounts/{}/orders/{}",
        client.client_options.api_url, client.client_options.account_id, order_id
    );

    let request_builder = client.client.patch(&url).json(&body);
    let response: reqwest::blocking::Response = request_builder
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text()?;
        return Err(Error::new(HttpError, &format!("Error: {} - {}", status, error_body)));
    }

    Ok(())
}
