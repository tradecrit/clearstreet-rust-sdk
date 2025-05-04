# Clear Street Rust SDK

[![License: AGPLv3](https://img.shields.io/badge/license-AGPLv3%2BNo%20Sale-blue)](#license)

Rust library providing an API interface to the public endpoints offered by Clear Street.

## Features

- Fully asynchronous Rust client based on [`reqwest`](https://docs.rs/reqwest/latest/reqwest/).
- Typed models for Clear Street public API requests and responses.
- Ready-to-integrate into trading systems, research tools, or financial applications.
- Minimal external dependencies for high performance.

## Installation

Add the SDK to your `Cargo.toml`:

```toml
[dependencies]
clearstreet_sdk = { git = "https://github.com/YOUR_USERNAME/clearstreet-rust-sdk.git", branch = "main" }
```

```rust
use clearstreet_sdk::ClearStreetClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = ClearStreetClient::new();

    // Fetch market data for a symbol
    let market_data = client.get_market_data("AAPL").await?;

    println!("{:?}", market_data);

    Ok(())
}
```

## Roadmap

- [x] Implement authenticated/private endpoints

- [ ] Add support for WebSocket streaming (if provided by Clear Street)

- [ ] Improve error handling and expose structured API errors

- [ ] Full test coverage
 
- [ ] Publish as an open-source crate

## Contributing

Contributions are welcome!

    Please open an issue to discuss any major changes before submitting a pull request.

    Code should pass cargo fmt and cargo clippy.

    All pull requests must include relevant tests where applicable.


## License

This project is licensed under a modified GNU Affero General Public License v3 (AGPLv3) with additional restrictions:

    You are allowed to use, modify, and integrate this software for commercial purposes.
    However, you are prohibited from selling the software itself, or modified versions thereof, as a standalone product or as part of any product offering.

For full license terms, see LICENSE.
