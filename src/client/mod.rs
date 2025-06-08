use std::fmt::{Debug, Display};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use crate::error::{Error, ErrorType};
use crate::orders;
use crate::orders::create::{CreateOrderParams, CreateOrderResponse};
use crate::orders::get::{list_orders, ListOrdersParams, ListOrdersResponse};
use crate::orders::Order;
use crate::orders::update::UpdateOrderRequestBody;
use crate::positions::{list_positions, ListPositionsResponse, Position};

#[cfg(feature = "async")]
pub mod async_client;

#[cfg(feature = "sync")]
pub mod sync_client;


pub trait ClearstreetClient {
    fn set_token(&mut self, token: &str);
    fn build_client(&self, token: &str) -> Result<reqwest::blocking::Client, Error>;
    fn create_order(&self, params: CreateOrderParams) -> Result<CreateOrderResponse, Error>;
}

pub fn build_headers(token: &str) -> Result<reqwest::header::HeaderMap, Error> {
    let mut headers = reqwest::header::HeaderMap::new();

    let bearer_token = reqwest::header::HeaderValue::from_str(token)
        .map_err(|e| Error::new(ErrorType::SerializationError, &e.to_string()))?;

    headers.insert(AUTHORIZATION, bearer_token);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert("Accept", "application/json".parse()?);

    Ok(headers)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClientOptions {
    pub api_url: String,
    pub websocket_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub account_id: String
}

impl Display for ClientOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientOptions {{ api_url: {}, websocket_url: {}, client_id: {}, client_secret: **REDACTED** }}",
            self.api_url, self.websocket_url, self.client_id
        )
    }
}

impl Debug for ClientOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientOptions {{ api_url: {}, websocket_url: {}, client_id: {}, client_secret: **REDACTED** }}",
            self.api_url, self.websocket_url, self.client_id
        )
    }
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            api_url: "https://api.clearstreet.io".to_string(),
            websocket_url: "wss://api.clearstreet.io/studio/v2/ws".to_string(),
            client_id: "<your_client_id>".to_string(),
            client_secret: "<your_client_secret>".to_string(),
            account_id: "<your_account_id>".to_string(),
        }
    }
}

#[cfg(feature = "async")]
pub trait AsyncClearstreetClient {
    fn set_token(&mut self, token: &str);
    fn build_client(&self, token: &str) -> Result<reqwest::Client, Error>;
    async fn create_order(&self, params: CreateOrderParams) -> Result<CreateOrderResponse, Error>;
    async fn get_order(&self, order_id: &str) -> Result<orders::Order, Error>;
    async fn update_order(&self, order_id: &str, params: UpdateOrderRequestBody) -> Result<(), Error>;
    async fn delete_order(&self, order_id: &str) -> Result<(), Error>;
    async fn delete_all_orders(&self, symbol: Option<&str>) -> Result<(), Error>;
    async fn list_orders(&self, params: ListOrdersParams) -> Result<ListOrdersResponse, Error>;
    async fn get_position(&self, symbol: &str) -> Result<Position, Error>;
    async fn list_positions(&self) -> Result<ListPositionsResponse, Error>;
}

#[cfg(feature = "sync")]
pub trait SyncClearstreetClient {
    fn set_token(&mut self, token: &str);
    fn build_client(&self, token: &str) -> Result<reqwest::blocking::Client, Error>;
    fn create_order(&self, params: CreateOrderParams) -> Result<CreateOrderResponse, Error>;
    fn get_order(&self, order_id: &str) -> Result<Order, Error>;
    fn update_order(&self, order_id: &str, params: UpdateOrderRequestBody) -> Result<(), Error>;
    fn list_orders(&self, params: ListOrdersParams) -> Result<ListOrdersResponse, Error>;
    fn delete_order(&self, order_id: &str) -> Result<(), Error>;
    fn delete_all_orders(&self, symbol: Option<&str>) -> Result<(), Error>;
    fn get_position(&self, symbol: &str) -> Result<Position, Error>;
    fn list_positions(&self) -> Result<ListPositionsResponse, Error>;
}