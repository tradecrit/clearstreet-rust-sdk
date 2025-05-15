use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::Client;
use crate::error::{Error, ErrorType};
use crate::orders::create::{CreateOrderParams, CreateOrderResponse};
use crate::orders::delete::{DeleteAllOrdersParams, DeleteOrderParams};
use crate::orders::get::GetOrderParams;
use crate::orders::Order;

#[async_trait]
pub trait ClientInterface: Send + Sync {
    async fn delete_order(&self, token: &str, params: DeleteOrderParams) -> Result<(), Error>;
    async fn delete_all_orders(&self, token: &str, params: DeleteAllOrdersParams) -> Result<(), Error>;
    async fn get_order(&self, _token: &str, params: GetOrderParams) -> Result<Order, Error>;
    async fn create_order(&self, token: &str, params: CreateOrderParams) -> Result<CreateOrderResponse, Error>;
}


#[async_trait]
impl ClientInterface for Client {
    async fn delete_order(&self, token: &str, params: DeleteOrderParams) -> Result<(), Error> {
        self.delete_order(token, params).await
    }

    async fn delete_all_orders(&self, token: &str, params: DeleteAllOrdersParams) -> Result<(), Error> {
        self.delete_all_orders(token, params).await
    }

    async fn get_order(&self, token: &str, params: GetOrderParams) -> Result<Order, Error> {
        self.get_order(token, params).await
    }

    async fn create_order(&self, token: &str, params: CreateOrderParams) -> Result<CreateOrderResponse, Error> {
        self.create_order(token, params).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct MockClient {
    pub deleted_order_ids: Arc<Mutex<Vec<(String, String)>>>,
    pub deleted_all_orders: Arc<Mutex<Vec<(String, Option<String>)>>>,
    pub orders: Arc<Mutex<HashMap<(String, String), Order>>>,
    pub created_orders: Arc<Mutex<Vec<CreateOrderParams>>>,
}

#[async_trait]
impl ClientInterface for MockClient {
    async fn delete_order(&self, _token: &str, params: DeleteOrderParams) -> Result<(), Error> {
        self.deleted_order_ids
            .lock()
            .await
            .push((params.account_id, params.order_id));
        Ok(())
    }

    async fn delete_all_orders(&self, _token: &str, params: DeleteAllOrdersParams) -> Result<(), Error> {
        self.deleted_all_orders
            .lock()
            .await
            .push((params.account_id, params.symbol));
        Ok(())
    }

    async fn get_order(&self, _token: &str, params: GetOrderParams) -> Result<Order, Error> {
        let key = (params.account_id.clone(), params.order_id.clone());
        let map = self.orders.lock().await;
        map.get(&key)
            .cloned()
            .ok_or_else(|| Error::new(ErrorType::NotFound, "Order not found".to_string()))
    }

    async fn create_order(&self, _token: &str, params: CreateOrderParams) -> Result<CreateOrderResponse, Error> {
        self.created_orders.lock().await.push(params.clone());
        Ok(CreateOrderResponse {
            order_id: "mock_order_123".to_string(),
        })
    }
}