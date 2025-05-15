use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use crate::Client;
use crate::error::Error;
use crate::error::ErrorType::HttpError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOrderParams {
    pub account_id: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAllOrdersParams {
    pub account_id: String,
    pub symbol: Option<String>,
}

impl Client {
    /// Deletes an order. NOTE this attempts to cancel the order, that doesn't mean that is does.
    /// The response status code then means the request was received and sent for cancellation.
    ///
    /// # Arguments
    ///
    /// * `params` - The order parameters
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Ok if the order was deleted, Err if there was an error
    ///
    pub async fn delete_order(&self, token: &str, params: DeleteOrderParams) -> Result<(), Error> {
        tracing::debug!("delete_order: {:?}", params);

        let client = self.build_authenticated_client(token).await?;

        let url: String = format!(
            "{}/studio/v2/accounts/{}/orders/{}",
            self.client_options.api_url, params.account_id, params.order_id
        );

        let request_builder: RequestBuilder = client.delete(&url);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(
            HttpError,
            format!("Error: {} - {}", status, error_body),
        ))
    }

    /// Deletes all orders. NOTE this attempts to cancel the orders, that doesn't mean that is does.
    /// The response status code then means the request was received and sent for cancellation.
    /// 
    /// # Arguments
    /// 
    /// * `params` - The order parameters
    /// 
    /// # Returns
    /// 
    /// * `Result<(), Error>` - Ok if the orders were deleted, Err if there was an error
    /// 
    pub async fn delete_all_orders(&self, token: &str, params: DeleteAllOrdersParams) -> Result<(), Error> {
        tracing::debug!("delete_all_orders: {:?}", params);

        let client = self.build_authenticated_client(token).await?;

        let url: String = format!(
            "{}/studio/v2/accounts/{}/orders",
            self.client_options.api_url, params.account_id
        );

        let query_params = if let Some(symbol) = params.symbol {
            format!("?symbol={}", symbol.to_uppercase())
        } else {
            String::new()
        };

        let url = format!("{}{}", url, query_params);

        let request_builder: RequestBuilder = client.delete(&url);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(
            HttpError,
            format!("Error: {} - {}", status, error_body),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClientOptions, Client};
    use mockito::Server;

    fn build_test_client(api_url: String) -> Client {
        Client::new(ClientOptions {
            client_id: "test_id".into(),
            client_secret: "test_secret".into(),
            api_url,
            ..Default::default()
        })
    }

    #[tokio::test]
    async fn test_delete_order_success() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("DELETE", "/studio/v2/accounts/test_account/orders/test_order")
            .with_status(204)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let result = client.delete_order(
            token,
            DeleteOrderParams {
                account_id: "test_account".into(),
                order_id: "test_order".into(),
            },
        ).await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_order_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("DELETE", "/studio/v2/accounts/test_account/orders/fail_order")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let result = client.delete_order(
            token,
            DeleteOrderParams {
                account_id: "test_account".into(),
                order_id: "fail_order".into(),
            },
        ).await;

        mock.assert_async().await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("500"));
        assert!(err.contains("Internal Server Error"));
    }

    #[tokio::test]
    async fn test_delete_all_orders_success_with_symbol() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("DELETE", "/studio/v2/accounts/test_account/orders?symbol=AAPL")
            .with_status(204)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let result = client.delete_all_orders(
            token,
            DeleteAllOrdersParams {
                account_id: "test_account".into(),
                symbol: Some("aapl".into()),
            },
        ).await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_all_orders_success_without_symbol() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("DELETE", "/studio/v2/accounts/test_account/orders")
            .with_status(204)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let result = client.delete_all_orders(
            token,
            DeleteAllOrdersParams {
                account_id: "test_account".into(),
                symbol: None,
            },
        ).await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_all_orders_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("DELETE", "/studio/v2/accounts/test_account/orders")
            .with_status(404)
            .with_body("Not Found")
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let result = client.delete_all_orders(
            token,
            DeleteAllOrdersParams {
                account_id: "test_account".into(),
                symbol: None,
            },
        ).await;

        mock.assert_async().await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("404"));
        assert!(err.contains("Not Found"));
    }
}
