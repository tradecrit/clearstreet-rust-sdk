use crate::error::ErrorType::HttpError;
use crate::utils;
use crate::utils::parse_response;
use crate::Client;
use crate::Error;
use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderState {
    Open,
    Rejected,
    Closed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    #[serde(rename = "stop-limit")]
    StopLimit,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
    #[serde(rename = "sell-short")]
    SellShort,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolFormat {
    Osi,
    #[default]
    Cms,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DirectMarketAccessStrategy {
    #[serde(rename = "type")]
    pub strategy_type: StrategyType,
    pub destination: Destination,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SmartOrderRouterStrategy {
    #[serde(rename = "type")]
    pub strategy_type: StrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<Urgency>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VolumeWeightedAveragePriceStrategy {
    #[serde(rename = "type")]
    pub strategy_type: StrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<Urgency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_percent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_percent: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeWeightedAveragePriceStrategy {
    #[serde(rename = "type")]
    pub strategy_type: StrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<Urgency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_percent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_percent: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PercentageOfVolumeStrategy {
    #[serde(rename = "type")]
    pub strategy_type: StrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<Urgency>,
    pub target_percent: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ArrivalPrice {
    #[serde(rename = "type")]
    pub strategy_type: StrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<Urgency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_percent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_percent: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DarkStrategy {
    #[serde(rename = "type")]
    pub strategy_type: StrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<Urgency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_percent: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Urgency {
    #[serde(rename = "super-passive")]
    SuperPassive,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "aggressive")]
    Aggressive,
    #[serde(rename = "super-aggressive")]
    SuperAggressive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StrategyType {
    #[serde(rename = "sor")]
    SmartOrderRoute, // Smart Order Router
    #[serde(rename = "dark")]
    Dark, // Dark Pool
    #[serde(rename = "ap")]
    ArrivalPrice, // Arrival Price
    #[serde(rename = "pov")]
    PercentageOfVolume, // Percentage of Volume
    #[serde(rename = "twap")]
    TimeWeightedAveragePrice, // Time Weighted Average Price
    #[serde(rename = "vwap")]
    VolumeWeightedAveragePrice, // Volume Weighted Average Price
    #[serde(rename = "dma")]
    DirectMarketAccess, // Direct Market Access
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Strategy {
    SmartOrderRoute(SmartOrderRouterStrategy), // Smart Order Router
    Dark(DarkStrategy), // Dark Pool
    ArrivalPrice(ArrivalPrice), // Arrival Price
    PercentageOfVolume(PercentageOfVolumeStrategy), // Percentage of Volume
    TimeWeightedAveragePrice(TimeWeightedAveragePriceStrategy), // Time Weighted Average Price
    VolumeWeightedAveragePrice(VolumeWeightedAveragePriceStrategy), // Volume Weighted Average Price
    DirectMarketAccess(DirectMarketAccessStrategy), // Direct Market Access
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Destination {
    Arcx, // NYSE ARCA
    Bats, // BATS Exchange
    Baty, // BATS Y Exchange
    Edga, // EDGA Exchange
    Edgx, // EDGX Exchange
    Eprl, // MIAX Pearl Equities
    Iexg, // Investors' Exchange
    Memx, // Members' Exchange
    Xase, // NYSE American
    Xbos, // NASDAQ BX Exchange
    Xcis, // NYSE National
    Xnms, // NASDAQ/NMS (Global Market)
    Xnys, // New York Stock Exchange
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub average_price: f64, // funny this is the only one that is a float
    pub filled_quantity: String,
    pub order_update_reason: String,
    pub text: String,
    pub strategy: Option<Strategy>,
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
    #[tracing::instrument(skip(self, params))]
    pub async fn create_order(&self, params: CreateOrderParams) -> Result<CreateOrderResponse, Error> {
        tracing::debug!("create_order: {:?}", params);

        let client = self.build_authenticated_client().await?;

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
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
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
    #[tracing::instrument(skip(self, params))]
    pub async fn get_order(&self, params: OrderParams) -> Result<Order, Error> {
        tracing::debug!("get_order: {:?}", params);

        let client = self.build_authenticated_client().await?;

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
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
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
    #[tracing::instrument(skip(self, params))]
    pub async fn delete_order(&self, params: OrderParams) -> Result<(), Error> {
        tracing::debug!("delete_order: {:?}", params);

        let client = self.build_authenticated_client().await?;

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
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
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
    #[tracing::instrument(skip(self, params, body))]
    pub async fn update_order(
        &self,
        params: OrderParams,
        body: UpdateOrderRequestBody,
    ) -> Result<(), Error> {
        tracing::debug!("update_order: {:?}", params);
        tracing::debug!("update_order_request: {:?}", body);

        let client = self.build_authenticated_client().await?;

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
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
    }

    #[tracing::instrument(skip(self, account_id, params))]
    pub async fn list_orders(
        &self,
        account_id: &str,
        params: ListOrdersParams,
    ) -> Result<ListOrdersResponse, Error> {
        tracing::debug!("get_orders");

        let client = self.build_authenticated_client().await?;

        let url: String = format!("{}/studio/v2/accounts/{}/orders",  self.client_options.api_url, account_id);

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
        Err(Error::new(HttpError, format!("Error: {} - {}", status, error_body)))
    }
}

#[cfg(test)]
mod tests {
    use crate::orders::{CreateOrderParams, Destination, DirectMarketAccessStrategy, ListOrdersParams, OrderParams, OrderSide, OrderType, StrategyType, TimeInForce, UpdateOrderRequestBody};
    use crate::Client;
    use mockito::Server;
    use tracing_subscriber::fmt::format::FmtSpan;
    use crate::orders::Strategy::DirectMarketAccess;

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

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let strategy = DirectMarketAccessStrategy {
            strategy_type: StrategyType::DirectMarketAccess,
            destination: Destination::Arcx,
        };

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
            symbol_format: Default::default(),
            strategy: DirectMarketAccess(strategy),
        };

        let result = client.create_order(params).await;
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

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let params = OrderParams {
            account_id: "100000".to_string(),
            order_id: "12390213".to_string(),
        };

        let result = client.get_order(params).await;
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

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let params = OrderParams {
            account_id: "100000".to_string(),
            order_id: "abc123".to_string(),
        };

        let result = client.delete_order(params).await;
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

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let order_params: OrderParams = OrderParams {
            account_id: "100000".to_string(),
            order_id: "abc123".to_string()
        };

        let params = UpdateOrderRequestBody {
            quantity: "1".to_string(),
            price: Some("10.00".to_string()),
            stop_price: None,
        };

        let result = client.update_order(order_params, params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_orders() {
        setup_tracing();

        let mut server = Server::new_async().await;

        let _mock = server
            .mock("GET", "/studio/v2/accounts/100000/orders?from=0&to=0&page_size=1&page_token=string")
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

        let client = Client::new_with_token(server.url(), "".to_string(), "test-token".into());

        let params = ListOrdersParams {
            from: 0,
            to: 0,
            page_size: 1,
            page_token: "string".to_string(),
        };

        let result = client.list_orders("100000", params).await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.data.len(), 1);
    }
}
