# Qwesty Examples

This directory contains examples demonstrating how to use the qwesty HTTP client library.

All examples use **httpbin.org** as the test API endpoint, which provides reliable and consistent responses for testing HTTP clients. This makes the examples more predictable and easier to understand.

## Running Examples

You can run any example using:

```bash
cargo run --example <example_name>
```

## Available Examples

### `basic_get.rs`
Demonstrates a simple GET request using httpbin.org/get. Shows the new `HttpResponse` API where you get a response object first, then call `.deserialize()` to parse the JSON.

```bash
cargo run --example basic_get
```

### `error_handling.rs`
Shows how to handle different types of errors that can occur during HTTP requests and deserialization:
- Successful requests with deserialization
- Network/connection errors 
- JSON deserialization errors

### `basic_post.rs`
Demonstrates a POST request with JSON body using httpbin.org/post. Shows how to send structured data and inspect the response.

```bash
cargo run --example basic_post
```

### `basic_post_empty.rs`
Shows how to make a POST request without a body using httpbin.org/post. Useful for webhooks or ping endpoints.

```bash
cargo run --example basic_post_empty
```

### `basic_put.rs`
Demonstrates a PUT request with JSON body using httpbin.org/put. Shows how to update resources.

```bash
cargo run --example basic_put
```

### `basic_delete.rs`
Shows a simple DELETE request using httpbin.org/delete. Demonstrates resource deletion without a request body.

```bash
cargo run --example basic_delete
```

### `response_inspection.rs`
Shows how to inspect response details (status code, success flag) before deserializing the JSON.

```bash
cargo run --example response_inspection
```

## API Changes

The library now uses a two-step process:

1. **Make the request**: `let response = get(url).await?;`
2. **Deserialize the response**: `let data = response.deserialize::<YourStruct>().await?;`

This provides more control over response handling and better error separation between network issues and deserialization problems.
