use std::env;
use clearstreet::client::async_client::AsyncClient;
use clearstreet::client::AsyncClearstreetClient;

#[tokio::test]
pub async fn test_get_order() {
    dotenvy::dotenv().ok();

    let client_options = clearstreet::client::ClientOptions {
        client_id: env::var("CLEARSTREET_CLIENT_ID").expect("CLEARSTREET_CLIENT_ID not set"),
        client_secret: env::var("CLEARSTREET_CLIENT_SECRET").expect("CLEARSTREET_CLIENT_SECRET not set"),
        account_id: env::var("CLEARSTREET_ACCOUNT_ID").expect("CLEARSTREET_ACCOUNT_ID not set"),
        ..Default::default()
    };

    let client = AsyncClient::create(client_options).await;
    let order_id = "";

    #[cfg(feature = "async")]
    {
        let get = client.get_order(order_id).await;
        println!("{:#?}", get);

        assert!(get.is_ok());

        let order = get.unwrap();

        assert_eq!(order.order_id, order_id);
    }
}
