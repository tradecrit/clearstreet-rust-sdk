use crate::error::Error;
use crate::error::ErrorType::HttpError;
use reqwest::{RequestBuilder, Response};

#[cfg(feature="async")]
use crate::client::async_client::AsyncClient;
#[cfg(feature="sync")]
use crate::client::sync_client::SyncClient;

#[cfg(feature = "async")]
pub async fn delete_order(
    client: &AsyncClient,
    order_id: &str,
) -> Result<(), Error> {
    let url: String = format!(
        "{}/studio/v2/accounts/{}/orders/{}",
        client.client_options.api_url, client.client_options.account_id, order_id
    );

    let request_builder: RequestBuilder = client.client.delete(&url);
    let response: Response = request_builder.send().await?;

    if let Err(e) = response.error_for_status() {
        return Err(Error::new(HttpError, &e.to_string()));
    }

    Ok(())
}

#[cfg(feature = "async")]
pub async fn delete_all_orders(
    client: &AsyncClient,
    symbol: Option<&str>,
) -> Result<(), Error> {
    let url: String = format!(
        "{}/studio/v2/accounts/{}/orders",
        client.client_options.api_url, client.client_options.account_id
    );

    let query_params = if let Some(symbol) = &symbol {
        format!("?symbol={}", symbol.to_uppercase())
    } else {
        String::new()
    };

    let url = format!("{}{}", url, query_params);

    let request_builder: RequestBuilder = client.client.delete(&url);
    let response: Response = request_builder.send().await?;

    if let Err(e) = response.error_for_status() {
        return Err(Error::new(HttpError, &e.to_string()));
    }

    Ok(())
}

#[cfg(feature = "sync")]
pub fn delete_order_blocking(
    client: &SyncClient,
    order_id: &str,
) -> Result<(), Error> {
    let url: String = format!(
        "{}/studio/v2/accounts/{}/orders/{}",
        client.client_options.api_url, client.client_options.account_id, order_id
    );

    let request_builder: reqwest::blocking::RequestBuilder = client.client.post(&url);
    let response: reqwest::blocking::Response = request_builder
        .send()
        .map_err(|e| Error::new(HttpError, &e.to_string()))?;

    if let Err(e) = response.error_for_status() {
        return Err(Error::new(HttpError, &e.to_string()));
    }

    Ok(())
}

#[cfg(feature = "sync")]
pub fn delete_all_orders_blocking(
    client: &SyncClient,
    symbol: Option<&str>
) -> Result<(), Error> {
    let url: String = format!(
        "{}/studio/v2/accounts/{}/orders",
        client.client_options.api_url, client.client_options.account_id
    );

    let query_params = if let Some(symbol) = &symbol {
        format!("?symbol={}", symbol.to_uppercase())
    } else {
        String::new()
    };

    let url = format!("{}{}", url, query_params);

    let request_builder: reqwest::blocking::RequestBuilder = client.client.post(&url);
    let response: reqwest::blocking::Response = request_builder
        .send()
        .map_err(|e| Error::new(HttpError, &e.to_string()))?;

    if let Err(e) = response.error_for_status() {
        return Err(Error::new(HttpError, &e.to_string()));
    }

    Ok(())
}
