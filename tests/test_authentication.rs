use std::env;
use dotenvy::dotenv;
use tracing_subscriber::fmt::format::FmtSpan;
use clearstreet::{Client, ClientOptions};

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
async fn test_get_accounts() {
    dotenv().ok().unwrap_or_default();

    setup_tracing();

    let options = ClientOptions {
        client_id: env::var("CLIENT_ID").unwrap().to_string(),
        client_secret: env::var("CLIENT_SECRET").unwrap().to_string(),
        ..Default::default()
    };

    let client = Client::init(options).await.expect("Failed to initialize client");

    let account = client.get_accounts().await;

    assert!(account.is_ok());

    let data = account.unwrap();

    println!("{:#?}", data);
}

#[tokio::test]
async fn test_get_account() {
    dotenv().ok().unwrap_or_default();

    setup_tracing();

    let options = ClientOptions {
        client_id: env::var("CLIENT_ID").unwrap().to_string(),
        client_secret: env::var("CLIENT_SECRET").unwrap().to_string(),
        ..Default::default()
    };

    let client = Client::init(options).await.expect("Failed to initialize client");
    
    let account_id = env::var("ACCOUNT_ID").unwrap_or("123".to_string());

    let account = client.get_account(&account_id).await;

    assert!(account.is_ok());

    let data = account.unwrap();

    println!("{:#?}", data);
}
