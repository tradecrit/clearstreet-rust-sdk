
use crate::orders::strategy::Strategy;
use crate::error::ErrorType::HttpError;
use crate::utils;
use crate::utils::parse_response;
use crate::Client;
use crate::Error;
use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use rust_decimal::Decimal;


pub mod strategy;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderState {
    Open,
    Rejected,
    Closed,
}

impl FromStr for OrderState {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(OrderState::Open),
            "rejected" => Ok(OrderState::Rejected),
            "closed" => Ok(OrderState::Closed),
            other => Err(crate::Error::new(
                crate::error::ErrorType::ParseError,
                format!("Invalid OrderState: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    New,
    #[serde(rename = "partially-filled")]
    PartiallyFilled,
    Filled,
    Canceled,
    Replaced,
    #[serde(rename = "pending-cancel")]
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    #[serde(rename = "pending-new")]
    PendingNew,
    Calculated,
    Expired,
    #[serde(rename = "accepted-for-bidding")]
    AcceptedForBidding,
    #[serde(rename = "pending-replace")]
    PendingReplace,
    #[serde(rename = "done-for-day")]
    DoneForDay,
}

impl FromStr for OrderStatus {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new" => Ok(OrderStatus::New),
            "partially-filled" => Ok(OrderStatus::PartiallyFilled),
            "filled" => Ok(OrderStatus::Filled),
            "canceled" => Ok(OrderStatus::Canceled),
            "replaced" => Ok(OrderStatus::Replaced),
            "pending-cancel" => Ok(OrderStatus::PendingCancel),
            "stopped" => Ok(OrderStatus::Stopped),
            "rejected" => Ok(OrderStatus::Rejected),
            "suspended" => Ok(OrderStatus::Suspended),
            "pending-new" => Ok(OrderStatus::PendingNew),
            "calculated" => Ok(OrderStatus::Calculated),
            "expired" => Ok(OrderStatus::Expired),
            "accepted-for-bidding" => Ok(OrderStatus::AcceptedForBidding),
            "pending-replace" => Ok(OrderStatus::PendingReplace),
            "done-for-day" => Ok(OrderStatus::DoneForDay),
            other => Err(crate::Error::new(
                crate::error::ErrorType::ParseError,
                format!("Invalid OrderStatus: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    #[serde(rename = "stop-limit")]
    StopLimit,
}

impl FromStr for OrderType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "market" => Ok(OrderType::Market),
            "limit" => Ok(OrderType::Limit),
            "stop" => Ok(OrderType::Stop),
            "stop-limit" => Ok(OrderType::StopLimit),
            other => Err(crate::Error::new(
                crate::error::ErrorType::ParseError,
                format!("Invalid OrderType: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    Buy,
    Sell,
    #[serde(rename = "sell-short")]
    SellShort,
}

impl FromStr for OrderSide {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy" => Ok(OrderSide::Buy),
            "sell" => Ok(OrderSide::Sell),
            "sell-short" => Ok(OrderSide::SellShort),
            other => Err(crate::Error::new(
                crate::error::ErrorType::ParseError,
                format!("Invalid OrderSide: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "day-plus")]
    DayPlus,
    #[serde(rename = "at-open")]
    AtOpen,
    #[serde(rename = "at-close")]
    AtClose,
}

impl FromStr for TimeInForce {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(TimeInForce::Day),
            "ioc" => Ok(TimeInForce::ImmediateOrCancel),
            "day-plus" => Ok(TimeInForce::DayPlus),
            "at-open" => Ok(TimeInForce::AtOpen),
            "at-close" => Ok(TimeInForce::AtClose),
            other => Err(crate::Error::new(
                crate::error::ErrorType::ParseError,
                format!("Invalid TimeInForce: {}", other),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SymbolFormat {
    Osi,
    #[default]
    Cms,
}

impl FromStr for SymbolFormat {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "osi" => Ok(SymbolFormat::Osi),
            "cms" => Ok(SymbolFormat::Cms),
            other => Err(crate::Error::new(
                crate::error::ErrorType::ParseError,
                format!("Invalid SymbolFormat: {}", other),
            )),
        }
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderParams {
    pub account_id: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrdersParams {
    pub from: i64,
    pub to: i64,
    pub page_size: i64,
    pub page_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequestBody {
    pub quantity: String,
    pub price: Option<String>,
    pub stop_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Order {
    pub created_at: i64,
    pub updated_at: i64,
    pub order_id: String,
    pub reference_id: String,
    pub version: i64,
    pub account_id: String,
    pub account_number: String,
    pub state: OrderState,
    pub status: OrderStatus,
    pub symbol: String,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub quantity: String,
    pub price: Option<String>,
    pub stop_price: Option<String>,
    pub time_in_force: TimeInForce,
    pub average_price: Decimal, // funny this is the only one that is a float
    pub filled_quantity: String,
    pub order_update_reason: String,
    pub text: String,
    pub strategy: Strategy,
    pub running_position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderResponse {
    pub order: Order,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrdersResponse {
    pub data: Vec<Order>,
    pub next_page_token: Option<String>,
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
    #[tracing::instrument(skip(self, token, params))]
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

        let response: Response = utils::request(request_builder).await?;

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
    #[tracing::instrument(skip(self, token, params))]
    pub async fn get_order(&self, token: &str, params: OrderParams) -> Result<Order, Error> {
        tracing::debug!("get_order: {:?}", params);

        let client = self.build_authenticated_client(token).await?;

        let url = format!(
            "{}/studio/v2/accounts/{}/orders/{}",
            self.client_options.api_url, params.account_id, params.order_id
        );

        let request_builder: RequestBuilder = client.get(&url);

        let response: Response = utils::request(request_builder).await?;

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
    #[tracing::instrument(skip(self, token, params))]
    pub async fn delete_order(&self, token: &str, params: OrderParams) -> Result<(), Error> {
        tracing::debug!("delete_order: {:?}", params);

        let client = self.build_authenticated_client(token).await?;

        let url: String = format!(
            "{}/studio/v2/accounts/{}/orders/{}",
            self.client_options.api_url, params.account_id, params.order_id
        );

        let request_builder: RequestBuilder = client.delete(&url);

        let response: Response = utils::request(request_builder).await?;

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
    #[tracing::instrument(skip(self, token, params, body))]
    pub async fn update_order(
        &self,
        token: &str,
        params: OrderParams,
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

        let response: Response = utils::request(request_builder).await?;

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

    #[tracing::instrument(skip(self, token, account_id, params))]
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

        let response: Response = utils::request(request_builder).await?;

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
    use crate::orders::{
        CreateOrderParams, ListOrdersParams, OrderParams, OrderSide, OrderType,
        SymbolFormat, TimeInForce, UpdateOrderRequestBody,
    };
    use crate::{Client, ClientOptions};
    use mockito::Server;
    use tracing_subscriber::fmt::format::FmtSpan;
    use crate::orders::strategy::{Destination, Strategy, Urgency};

    fn setup_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::new("DEBUG"))
            .with_target(true)
            .with_level(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_line_number(true)
            .with_ansi(true)
            .with_writer(std::io::stdout)
            .try_init();
    }

    #[tokio::test]
    async fn test_create_order() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("POST", "/studio/v2/accounts/100000/orders")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "order_id": "abc123"
            }"#,
            )
            .create_async()
            .await;

        let options = ClientOptions {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            ..Default::default()
        };

        let client = Client::new(options);
        let token = client.fetch_new_token().await.unwrap();

        let params = CreateOrderParams {
            account_id: "100000".to_string(),
            reference_id: "my-custom-id".to_string(),
            order_type: OrderType::Limit,
            order_side: OrderSide::Buy,
            quantity: "1".to_string(),
            price: Some("10.00".to_string()),
            stop_price: None,
            time_in_force: TimeInForce::Day,
            symbol: "AAPL".to_string(),
            symbol_format: SymbolFormat::Cms,
            strategy: Strategy::SmartOrderRoute {
                start_at: None,
                end_at: None,
                urgency: Some(Urgency::Moderate),
            },
        };

        let result = client.create_order(&token.access_token, params).await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.order_id, "abc123");
    }

    #[tokio::test]
    async fn test_get_order() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("GET", "/studio/v2/accounts/100000/orders/12390213")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "order": {
                        "created_at": 0,
                        "updated_at": 0,
                        "order_id": "12390213",
                        "reference_id": "my-order-id",
                        "version": 1,
                        "account_id": "100000",
                        "account_number": "ACC0001",
                        "state": "open",
                        "status": "new",
                        "symbol": "AAPL",
                        "order_type": "limit",
                        "side": "buy",
                        "quantity": "100",
                        "price": "123.99",
                        "stop_price": "123.99",
                        "time_in_force": "day",
                        "average_price": 0,
                        "filled_quantity": "100",
                        "order_update_reason": "place",
                        "text": "string",
                        "strategy": {
                            "type": "sor",
                            "start_at": 0,
                            "end_at": 0,
                            "urgency": "moderate"
                        },
                        "running_position": "100"
                    }
                }
            "#,
            )
            .create_async()
            .await;

        let options = ClientOptions {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            ..Default::default()
        };

        let client = Client::new(options);
        let token = client.fetch_new_token().await.unwrap();

        let params = OrderParams {
            account_id: "100000".to_string(),
            order_id: "12390213".to_string(),
        };

        let result = client.get_order(&token.access_token, params).await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.order_id, "12390213");
    }

    #[tokio::test]
    async fn test_delete_order() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("DELETE", "/studio/v2/accounts/100000/orders/abc123")
            .with_status(201)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let options = ClientOptions {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            ..Default::default()
        };

        let client = Client::new(options);
        let token = client.fetch_new_token().await.unwrap();

        let params = OrderParams {
            account_id: "100000".to_string(),
            order_id: "abc123".to_string(),
        };

        let result = client.delete_order(&token.access_token, params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_order() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("PATCH", "/studio/v2/accounts/100000/orders/abc123")
            .with_status(201)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let options = ClientOptions {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            ..Default::default()
        };

        let client = Client::new(options);
        let token = client.fetch_new_token().await.unwrap();

        let order_params: OrderParams = OrderParams {
            account_id: "100000".to_string(),
            order_id: "abc123".to_string(),
        };

        let params = UpdateOrderRequestBody {
            quantity: "1".to_string(),
            price: Some("10.00".to_string()),
            stop_price: None,
        };

        let result = client
            .update_order(&token.access_token, order_params, params)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_orders() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock(
                "GET",
                "/studio/v2/accounts/100000/orders?from=0&to=0&page_size=1&page_token=string",
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                  "data": [
                    {
                      "created_at": 0,
                      "updated_at": 0,
                      "order_id": "12390213",
                      "reference_id": "my-order-id-123",
                      "version": 1,
                      "account_id": "100000",
                      "account_number": "ACC0001",
                      "state": "open",
                      "status": "new",
                      "symbol": "AAPL",
                      "order_type": "limit",
                      "side": "buy",
                      "quantity": "100",
                      "price": "123.99",
                      "stop_price": "123.99",
                      "time_in_force": "day",
                      "average_price": 0,
                      "filled_quantity": "100",
                      "order_update_reason": "place",
                      "text": "string",
                      "strategy": {
                        "type": "sor",
                        "start_at": 0,
                        "end_at": 0,
                        "urgency": "moderate"
                      },
                      "running_position": "100"
                    }
                  ],
                  "next_page_token": "string"
                }
            "#,
            )
            .create_async()
            .await;

        let options = ClientOptions {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            ..Default::default()
        };

        let client = Client::new(options);
        let token = client.fetch_new_token().await.unwrap();

        let params = ListOrdersParams {
            from: 0,
            to: 0,
            page_size: 1,
            page_token: "string".to_string(),
        };

        let result = client
            .list_orders(&token.access_token, "100000", params)
            .await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.data.len(), 1);
    }
}
