use std::env;
use dotenvy::dotenv;
use tracing_subscriber::fmt::format::FmtSpan;
use clearstreet::{Client, ClientOptions};
use clearstreet::orders::{CreateOrderParams, OrderSide, OrderType, SmartOrderRouterStrategy, Strategy, StrategyType, TimeInForce};
use uuid::Uuid;

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
    dotenv().ok().unwrap_or_default();

    setup_tracing();

    let options = ClientOptions {
        client_id: env::var("CLIENT_ID").unwrap().to_string(),
        client_secret: env::var("CLIENT_SECRET").unwrap().to_string(),
        ..Default::default()
    };

    let client = Client::init(options).await.expect("Failed to initialize client");

    let strategy = SmartOrderRouterStrategy {
        strategy_type: StrategyType::SmartOrderRoute,
        start_at: None,
        end_at: None,
        urgency: None,
    };

    let params = CreateOrderParams {
        account_id: env::var("ACCOUNT_ID").unwrap().to_string(),
        reference_id: Uuid::now_v7().to_string(),
        order_type: OrderType::Limit,
        order_side: OrderSide::Buy,
        quantity: "1".to_string(),
        price: Some("100.00".to_string()),
        stop_price: None,
        time_in_force: TimeInForce::Day,
        symbol: "COST".to_string(),
        symbol_format: Default::default(),
        strategy: Strategy::SmartOrderRoute(strategy),
    };

    let result = client.create_order(params).await;

    if let Err(err) = result {
        tracing::error!(error = ?err, "Error creating order");
        panic!("Error creating order: {:?}", err);
    }

    let data = result.unwrap();

    println!("{:#?}", data);
}
