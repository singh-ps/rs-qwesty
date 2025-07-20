# qwesty

A Rust HTTP client library that provides a simplified wrapper around `reqwest` with built-in error handling and JSON deserialization.

## Features

- Simple HTTP GET and PUT operations
- Automatic JSON deserialization using serde
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
    let assets = get::<Assets>("https://api.example.com/assets").await?;
    println!("Fetched {} assets", assets.assets.len());
    Ok(())
}
```

## Examples

See the `examples/` directory for more detailed usage examples:

- `basic_get.rs` - Simple GET request example
- `error_handling.rs` - Error handling demonstration
- `basic_put.rs` - PUT request example

Run examples with:
```bash
cargo run --example basic_get
```

## Error Handling

The library provides three main error types:

- `RequestFailed` - HTTP request failures
- `DeSerError` - JSON deserialization errors
- `ClientError` - HTTP client creation errors

## Testing

Run tests with:
```bash
cargo test
```
