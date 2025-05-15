use crate::error::Error;
use crate::error::ErrorType::HttpError;
use crate::orders::{Order};
use crate::utils::parse_response;
use crate::Client;
use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderResponse {
    pub order: Order,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderParams {
    pub account_id: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrdersResponse {
    pub data: Vec<Order>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrdersParams {
    pub from: i64,
    pub to: i64,
    pub page_size: i64,
    pub page_token: String,
}

impl Client {
    /// Gets an order by id. There is currently a problem with the documents that show
    /// incompatible types for the strategy route.
    ///
    /// # Arguments
    ///
    /// * `params` - The order parameters
    ///
    /// # Returns
    ///
    /// * `Result<Order, Error>` - Ok if the order was found, Err if there was an error
    ///
    pub async fn get_order(&self, token: &str, params: GetOrderParams) -> Result<Order, Error> {
        tracing::debug!("get_order: {:?}", params);

        let client = self.build_authenticated_client(token).await?;

        let url = format!(
            "{}/studio/v2/accounts/{}/orders/{}",
            self.client_options.api_url, params.account_id, params.order_id
        );

        let request_builder: RequestBuilder = client.get(&url);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: GetOrderResponse = parse_response(response).await?;
            tracing::debug!("{:?}", body);
            return Ok(body.order);
        }

        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::new(
            HttpError,
            format!("Error: {} - {}", status, error_body),
        ))
    }

    pub async fn list_orders(
        &self,
        token: &str,
        account_id: &str,
        params: ListOrdersParams,
    ) -> Result<ListOrdersResponse, Error> {
        tracing::debug!("get_orders");

        let client = self.build_authenticated_client(token).await?;

        let url: String = format!(
            "{}/studio/v2/accounts/{}/orders",
            self.client_options.api_url, account_id
        );

        let request_builder: RequestBuilder = client
            .get(&url)
            .query(&[("from", params.from)])
            .query(&[("to", params.to)])
            .query(&[("page_size", params.page_size)])
            .query(&[("page_token", params.page_token)]);

        let response: Response = request_builder
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: ListOrdersResponse = parse_response(response).await?;
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
    use crate::orders::get::GetOrderParams;
    use rust_decimal_macros::dec;
    use crate::{ClientInterface, MockClient};
    use crate::orders::{Order, OrderSide, OrderState, OrderStatus, OrderType, TimeInForce};
    use crate::orders::strategy::Strategy::SmartOrderRoute;

    #[tokio::test]
    async fn test_get_order_mock() {
        let mock = MockClient::default();

        let expected_order = Order {
            created_at: 0,
            updated_at: 0,
            order_id: "ord123".to_string(),
            reference_id: "".to_string(),
            version: 0,
            account_id: "acct456".to_string(),
            account_number: "".to_string(),
            state: OrderState::Open,
            status: OrderStatus::New,
            symbol: "AAPL".to_string(),
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            quantity: dec!(100).to_string(),
            price: None,
            stop_price: None,
            time_in_force: TimeInForce::Day,
            average_price: Default::default(),
            filled_quantity: "".to_string(),
            order_update_reason: "".to_string(),
            text: "".to_string(),
            strategy: SmartOrderRoute {
                urgency: None,
                end_at: None,
                start_at: None,
            },
            running_position: "".to_string(),
        };

        {
            let mut orders = mock.orders.lock().await;
            orders.insert(
                ("acct456".to_string(), "ord123".to_string()),
                expected_order.clone(),
            );
        }

        let result = mock
            .get_order("fake_token", GetOrderParams {
                account_id: "acct456".to_string(),
                order_id: "ord123".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(result.order_id, expected_order.order_id);
    }
}