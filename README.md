# qwesty

A Rust HTTP client library that provides a simplified wrapper around `reqwest` with built-in error handling and JSON deserialization.

## Features

- Simple HTTP GET, POST, PUT, and DELETE operations
- Empty POST requests (no body)
- Two-step response handling (get response, then deserialize)
- JSON request bodies for POST and PUT operations
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
use qwesty::http::{get, post, post_empty, put, delete};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
struct CreateRequest {
    name: String,
    version: String,
}

#[derive(Serialize)]
struct UpdateRequest {
    name: String,
    version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET request
    let response = get("https://httpbin.org/get").await?;
    let data = response.deserialize::<serde_json::Value>().await?;
    println!("GET response: {}", data);
    
    // POST request to create new resource
    let new_data = CreateRequest {
        name: "new-asset".to_string(),
        version: "1.0.0".to_string(),
    };
    let response = post("https://httpbin.org/post", &new_data).await?;
    let created_data = response.deserialize::<serde_json::Value>().await?;
    println!("POST response: {}", created_data);
    
    // PUT request to update existing resource
    let update_data = UpdateRequest {
        name: "updated-asset".to_string(),
        version: "2.0.0".to_string(),
    };
    let response = put("https://httpbin.org/put", &update_data).await?;
    let updated_data = response.deserialize::<serde_json::Value>().await?;
    println!("PUT response: {}", updated_data);
    
    // Empty POST request (no body)
    let response = post_empty("https://httpbin.org/post").await?;
    if response.is_success() {
        println!("Empty POST successful!");
    }
    
    // DELETE request to remove a resource
    let response = delete("https://httpbin.org/delete").await?;
    if response.is_success() {
        println!("DELETE request successful");
    }
    
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
