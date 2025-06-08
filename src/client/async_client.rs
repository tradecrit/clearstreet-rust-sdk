use std::time::Duration;
use crate::client::{build_headers, AsyncClearstreetClient, ClientOptions};
use crate::error::Error;
use crate::{authentication, orders};
use crate::authentication::TokenResponse;
use crate::orders::create::{CreateOrderParams, CreateOrderResponse};
use crate::orders::get::{list_orders, ListOrdersParams, ListOrdersResponse};
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

    async fn fetch_new_token(&self) -> Result<TokenResponse, Error> {
        authentication::fetch_new_token(self).await
    }

    fn build_client(&self, token: &str) -> Result<reqwest::Client, Error> {
        let headers = build_headers(token)?;
        reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .default_headers(headers)
            .build()
            .map_err(Error::from)
    }

    async fn create_order(&self, params: CreateOrderParams) -> Result<CreateOrderResponse, Error> {
        orders::create::create_order(self, params).await
    }

    async fn get_order(&self, order_id: &str) -> Result<orders::Order, Error> {
        orders::get::get_order(self, order_id).await
    }

    async fn update_order(&self, order_id: &str, params: UpdateOrderRequestBody) -> Result<(), Error> {
        update_order(self, order_id, params).await
    }

    async fn delete_order(&self, order_id: &str) -> Result<(), Error> {
        orders::delete::delete_order(self, order_id).await
    }
    
    async fn delete_all_orders(&self, symbol: Option<&str>) -> Result<(), Error> {
        orders::delete::delete_all_orders(self, symbol).await
    }

    async fn list_orders(&self, params: ListOrdersParams) -> Result<ListOrdersResponse, Error> {
        list_orders(self, params).await
    }

    async fn get_position(&self, symbol: &str) -> Result<Position, Error> {
        get_position(self, symbol).await
    }

    async fn list_positions(&self) -> Result<ListPositionsResponse, Error> {
        list_positions(self).await
    }
}
