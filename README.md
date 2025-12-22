# qwesty ![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/singh-ps/rs-qwesty) ![Rust](https://img.shields.io/badge/rust-2021-orange)

A Rust HTTP client library that provides a simplified wrapper around `reqwest` with built-in error handling, JSON deserialization, and automatic retries.

## Features

- Simple HTTP GET, POST, PUT, and DELETE operations
- **Configurable Client**: Custom timeouts, headers, and User-Agents.
- **Resilience**: Automatic retries with exponential backoff for transient failures.
- Empty POST requests (no body).
- Two-step response handling (get response, then deserialize).
- JSON request bodies for POST and PUT operations.
- Comprehensive error handling with custom error types.
- Built on top of the reliable `reqwest` and `reqwest-middleware` crates.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
qwesty = { git = "https://github.com/singh-ps/rs-qwesty", tag = "v0.1.0" }
```

> Note: Replace the tag with the latest version shown in the badge above.

## Basic Usage

For simple requests, you can pass `None` as the second argument to use the library's default configuration (60s timeout, default User-Agent, and no retries).

```rust
use qwesty::http::{get, post, post_empty, put, delete};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET request with default client
    let response = get("https://httpbin.org/get", None).await?;
    let data = response.deserialize::<serde_json::Value>().await?;
    println!("GET response: {}", data);

    // POST request with default client
    let new_data = serde_json::json!({"name": "new-asset"});
    let response = post("https://httpbin.org/post", None, &new_data).await?;

    Ok(())
}
```

## Advanced Usage: Configuration & Retries

You can create a `ConfiguredClient` to customize behavior and enable automatic retries for transient errors (like 500 or 503 status codes).

```rust
use qwesty::http::{get, ConfiguredClient};
use qwesty::models::{ClientConfig, RetryPolicy, Headers};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define a Retry Policy (e.g., retry 3 times)
    let retry_policy = RetryPolicy {
        retries: 3,
        min_interval: Duration::from_millis(100),
        max_interval: Duration::from_secs(2),
    };

    // 2. Create a Configuration
    let config = ClientConfig::new(
        30,                                // Timeout in seconds
        "My-Custom-App/1.0".to_string(),   // Custom User Agent
        Headers::default(),                // Headers
        Some(retry_policy),
    );

    // 3. Initialize the Client
    let client = ConfiguredClient::try_new(&config)?;

    // 4. Use the client for multiple requests
    let response = get("https://httpbin.org/get", Some(client.clone())).await?;

    Ok(())
}
```

## Examples

See the `examples/` directory for more detailed usage examples:

- `basic_get.rs` - Simple GET request example
- `basic_post.rs` - POST request to create new resource
- `basic_post_empty.rs` - Empty POST request (no body)
- `basic_put.rs` - PUT request to update existing resource
- `basic_delete.rs` - DELETE request to remove resource
- `error_handling.rs` - Error handling demonstration
- `response_inspection.rs` - Response inspection before deserialization

Run examples with:

```bash
cargo run --example basic_get
```

## Error Handling

The library provides four main error types:

- `RequestFailed` - HTTP request failures (includes status code and body context)
- `DeSerError` - JSON deserialization errors
- `ClientError` - HTTP client creation errors
- `HeaderError` - Error parsing or setting HTTP headers

## Testing

Run tests with:

```bash
cargo test
```
