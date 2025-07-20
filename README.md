# qwesty

A Rust HTTP client library that provides a simplified wrapper around `reqwest` with built-in error handling and JSON deserialization.

## Features

- Simple HTTP GET operations
- Two-step response handling (get response, then deserialize)
- Comprehensive error handling with custom error types
- Built on top of the reliable `reqwest` crate

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
qwesty = "0.0.1"
```

## Basic Usage

```rust
use qwesty::http::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct Assets {
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    version: String,
    // ... other fields
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Make the HTTP request
    let response = get("https://api.example.com/assets").await?;
    
    // Step 2: Deserialize the response
    let assets = response.deserialize::<Assets>().await?;
    
    println!("Fetched {} assets", assets.assets.len());
    Ok(())
}
```

## Examples

See the `examples/` directory for more detailed usage examples:

- `basic_get.rs` - Simple GET request example
- `error_handling.rs` - Error handling demonstration

Run examples with:
```bash
cargo run --example basic_get
```

## Error Handling

The library provides three main error types:

- `RequestFailed` - HTTP request failures
- `DeSerError` - JSON deserialization errors
- `ClientError` - HTTP client creation errors

Errors can occur at two levels:
1. **Request level** - Network issues, HTTP errors
2. **Deserialization level** - JSON parsing issues

## Testing

Run tests with:
```bash
cargo test
```
