use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use crate::Client;
use crate::error::Error;
use crate::error::ErrorType::HttpError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderParams {
    pub account_id: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequestBody {
    pub quantity: String,
    pub price: Option<String>,
    pub stop_price: Option<String>,
}

impl Client {
    /// Updates an order. The order is updated in the broker's system and is not
    /// immediately executed. The order is updated in the broker's system and is not
    /// immediately matched.
    ///
    /// # Arguments
    ///
    /// * `params` - The order parameters
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Ok if the order was updated, Err if there was an error
    ///
    pub async fn update_order(
        &self,
        token: &str,
        params: UpdateOrderParams,
        body: UpdateOrderRequestBody,
    ) -> Result<(), Error> {
        tracing::debug!("update_order: {:?}", params);
        tracing::debug!("update_order_request: {:?}", body);

        let client = self.build_authenticated_client(token).await?;

        let url: String = format!(
            "{}/studio/v2/accounts/{}/orders/{}",
            self.client_options.api_url, params.account_id, params.order_id
        );

        let request_builder: RequestBuilder = client.patch(&url).json(&body);

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
    
    use crate::{Client, ClientOptions};
    use crate::orders::update::{UpdateOrderParams, UpdateOrderRequestBody};
    use mockito::Server;

    fn build_test_client(api_url: String) -> Client {
        Client::new(ClientOptions {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            api_url,
            ..Default::default()
        })
    }

    #[tokio::test]
    async fn test_update_order_success() {
        let mut server = Server::new_async().await;

        let _mock = server
            .mock("PATCH", "/studio/v2/accounts/100000/orders/abc123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let params = UpdateOrderParams {
            account_id: "100000".to_string(),
            order_id: "abc123".to_string(),
        };

        let body = UpdateOrderRequestBody {
            quantity: "1".to_string(),
            price: Some("10.00".to_string()),
            stop_price: None,
        };

        let result = client.update_order(token, params, body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_order_failure() {
        let mut server = Server::new_async().await;

        let _mock = server
            .mock("PATCH", "/studio/v2/accounts/100000/orders/abc123")
            .with_status(400)
            .with_body("Bad Request")
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let params = UpdateOrderParams {
            account_id: "100000".to_string(),
            order_id: "abc123".to_string(),
        };

        let body = UpdateOrderRequestBody {
            quantity: "0".to_string(), // simulate invalid quantity
            price: Some("10.00".to_string()),
            stop_price: None,
        };

        let result = client.update_order(token, params, body).await;
        assert!(result.is_err());

        let error = result.unwrap_err().to_string();
        assert!(error.contains("400"));
        assert!(error.contains("Bad Request"));
    }
}
