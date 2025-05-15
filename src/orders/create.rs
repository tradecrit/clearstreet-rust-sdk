use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use crate::Client;
use crate::error::Error;
use crate::error::ErrorType::HttpError;
use crate::orders::{OrderSide, OrderType, SymbolFormat, TimeInForce};
use crate::orders::strategy::Strategy;
use crate::utils::parse_response;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderParams {
    pub account_id: String,
    pub reference_id: String,
    pub order_type: OrderType,
    #[serde(rename = "side")]
    pub order_side: OrderSide,
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    pub time_in_force: TimeInForce,
    pub symbol: String,
    pub symbol_format: SymbolFormat,
    pub strategy: Strategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

impl Client {
    /// Creates an order. The order is created in the broker's system and is not
    /// immediately executed. The order is created in the broker's system and is not
    /// immediately matched.
    ///
    /// # Arguments
    ///
    /// * `params` - The order parameters
    ///
    /// # Returns
    ///
    /// * `Result<CreateOrderResponse, Error>` - Ok if the order was created, Err if there was an error
    ///
    pub async fn create_order(
        &self,
        token: &str,
        params: CreateOrderParams,
    ) -> Result<CreateOrderResponse, Error> {
        tracing::debug!("create_order: {:?}", params);

        let client = self.build_authenticated_client(token).await?;

        let url = format!(
            "{}/studio/v2/accounts/{}/orders",
            self.client_options.api_url, params.account_id
        );

        let request_builder: RequestBuilder = client.post(&url).json(&params);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: CreateOrderResponse = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body);
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
    use crate::{Client, ClientOptions, MockClient};
    use crate::orders::{OrderSide, OrderType, SymbolFormat, TimeInForce};
    use crate::orders::strategy::Strategy;
    use mockito::Server;
    use crate::ClientInterface;

    fn build_test_client(api_url: String) -> Client {
        Client::new(ClientOptions {
            client_id: "test_client_id".into(),
            client_secret: "test_client_secret".into(),
            api_url,
            ..Default::default()
        })
    }

    fn sample_order_params() -> CreateOrderParams {
        CreateOrderParams {
            account_id: "acct123".into(),
            reference_id: "ref456".into(),
            order_type: OrderType::Limit,
            order_side: OrderSide::Buy,
            quantity: "100".into(),
            price: Some("10.50".into()),
            stop_price: None,
            time_in_force: TimeInForce::Day,
            symbol: "AAPL".into(),
            symbol_format: SymbolFormat::Cms,
            strategy: Strategy::SmartOrderRoute {
                start_at: None,
                end_at: None,
                urgency: None,
            },
        }
    }

    #[tokio::test]
    async fn test_create_order_success() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/studio/v2/accounts/acct123/orders")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_body(r#"{"order_id": "order789"}"#)
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let result = client.create_order(token, sample_order_params()).await;

        mock.assert_async().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().order_id, "order789");
    }

    #[tokio::test]
    async fn test_create_order_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/studio/v2/accounts/acct123/orders")
            .with_status(400)
            .with_body("Bad Request")
            .create_async()
            .await;

        let client = build_test_client(server.url());
        let token = "fake_token";

        let result = client.create_order(token, sample_order_params()).await;

        mock.assert_async().await;
        assert!(result.is_err());

        let err_str = result.unwrap_err().to_string();
        assert!(err_str.contains("400"));
        assert!(err_str.contains("Bad Request"));
    }

    #[tokio::test]
    async fn test_create_order_mock() {


        let mock = MockClient::default();
        let token = "fake_token";

        let params = CreateOrderParams {
            account_id: "acct123".into(),
            reference_id: "ref456".into(),
            order_type: OrderType::Limit,
            order_side: OrderSide::Buy,
            quantity: "100".into(),
            price: Some("10.50".into()),
            stop_price: None,
            time_in_force: TimeInForce::Day,
            symbol: "AAPL".into(),
            symbol_format: SymbolFormat::Cms,
            strategy: Default::default(),
        };

        let response = mock.create_order(token, params.clone()).await.unwrap();
        assert_eq!(response.order_id, "mock_order_123");

        let created = mock.created_orders.lock().await;
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].reference_id, "ref456");
    }
}
