use crate::client::{build_headers, ClientOptions, SyncClearstreetClient};
use crate::error::Error;
use crate::orders::create::{CreateOrderParams, CreateOrderResponse};
use crate::orders::delete::{delete_all_orders_blocking, delete_order_blocking};
use crate::orders::get::ListOrdersParams;
use crate::positions::ListPositionsResponse;
use crate::{authentication, orders, positions};
use std::time::Duration;
use crate::authentication::TokenResponse;

#[derive(Debug, Clone)]
pub struct SyncClient {
    pub client: reqwest::blocking::Client,
    pub client_options: ClientOptions,
    pub token: String,
}

#[cfg(feature = "sync")]
impl SyncClearstreetClient for SyncClient {
    fn set_token(&mut self, token: &str) {
        self.token = token.to_string();
    }

    fn fetch_new_token_blocking(&self) -> Result<TokenResponse, Error> {
        authentication::fetch_new_token_blocking(self)
    }

    fn build_client(&self, token: &str) -> Result<reqwest::blocking::Client, Error> {
        let headers = build_headers(token)?;
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .default_headers(headers)
            .build()
            .map_err(Error::from)
    }

    fn create_order(&self, params: CreateOrderParams) -> Result<CreateOrderResponse, Error> {
        orders::create::create_order_blocking(self, params)
    }

    fn get_order(&self, order_id: &str) -> Result<orders::Order, Error> {
        orders::get::get_order_blocking(self, order_id)
    }
    
    fn update_order(&self, order_id: &str, params: orders::update::UpdateOrderRequestBody) -> Result<(), Error> {
        orders::update::update_order_blocking(self, order_id, params)
    }

    fn list_orders(&self, params: ListOrdersParams) -> Result<orders::get::ListOrdersResponse, Error> {
        orders::get::list_orders_blocking(self, params)
    }

    fn delete_order(&self, order_id: &str) -> Result<(), Error> {
        delete_order_blocking(self, order_id)
    }

    fn delete_all_orders(&self, symbol: Option<&str>) -> Result<(), Error> {
        delete_all_orders_blocking(self, symbol)
    }

    fn get_position(&self, symbol: &str) -> Result<positions::Position, Error> {
        positions::get_position_blocking(self, symbol)
    }

    fn list_positions(&self) -> Result<ListPositionsResponse, Error> {
        positions::list_positions_blocking(self)
    }
}