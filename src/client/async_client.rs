use std::any::Any;
use std::time::Duration;
use crate::client::{build_headers, AsyncClearstreetClient, ClientOptions};
use crate::error::Error;
use crate::{authentication, orders};
use crate::authentication::TokenResponse;
use crate::orders::create::{CreateOrderParams, CreateOrderResponse};
use crate::orders::get::{list_orders, GetOrderResponse, ListOrdersParams, ListOrdersResponse};
use crate::orders::Order;
use crate::orders::update::{update_order, UpdateOrderRequestBody};
use crate::positions::{get_position, list_positions, ListPositionsResponse, Position};

#[derive(Debug, Clone)]
pub struct AsyncClient {
    pub client: reqwest::Client,
    pub client_options: ClientOptions,
    pub token: String,
}

#[cfg(feature = "async")]
impl AsyncClearstreetClient for AsyncClient {
    fn set_token(&mut self, token: &str) {
        self.token = token.to_string();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn build_client(&self, token: &str) -> Result<reqwest::Client, Error> {
        let headers = build_headers(token)?;
        reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .default_headers(headers)
            .build()
            .map_err(Error::from)
    }

    fn fetch_new_token(&self) -> impl std::future::Future<Output = Result<TokenResponse, Error>> {
        authentication::fetch_new_token(self)
    }

    fn create_order(&self, params: CreateOrderParams) -> impl Future<Output = Result<CreateOrderResponse, Error>> {
        orders::create::create_order(self, params)
    }

    fn get_order(&self, order_id: &str) -> impl Future<Output = Result<Order, Error>> {
        orders::get::get_order(self, order_id)
    }

    fn update_order(&self, order_id: &str, params: UpdateOrderRequestBody) -> impl Future<Output = Result<(), Error>> {
        update_order(self, order_id, params)
    }

    fn delete_order(&self, order_id: &str) -> impl Future<Output = Result<(), Error>> {
        orders::delete::delete_order(self, order_id)
    }

    fn delete_all_orders(&self, symbol: Option<&str>) -> impl Future<Output = Result<(), Error>> {
        orders::delete::delete_all_orders(self, symbol)
    }

    fn list_orders(&self, params: ListOrdersParams) -> impl Future<Output = Result<ListOrdersResponse, Error>> {
        list_orders(self, params)
    }

    fn get_position(&self, symbol: &str) -> impl Future<Output = Result<Position, Error>> {
        get_position(self, symbol)
    }

    fn list_positions(&self) -> impl Future<Output = Result<ListPositionsResponse, Error>> {
        list_positions(self)
    }
}
