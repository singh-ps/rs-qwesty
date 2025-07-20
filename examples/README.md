# Qwesty Examples

This directory contains examples demonstrating how to use the qwesty HTTP client library.

## Running Examples

You can run any example using:

```bash
cargo run --example <example_name>
```

## Available Examples

### `basic_get.rs`
Demonstrates a simple GET request to fetch and display assets. Defines its own `Assets` and `Asset` structs for the JSON response.

```bash
cargo run --example basic_get
```

### `error_handling.rs`
Shows how to handle different types of errors that can occur during HTTP requests:
- Successful requests
- Network/connection errors
- JSON deserialization errors

```bash
cargo run --example error_handling
```

### `basic_put.rs`
Demonstrates a basic PUT request (without request body for simplicity).

```bash
cargo run --example basic_put
```

## Notes

- The examples use real APIs where possible to demonstrate actual functionality
- Error handling examples intentionally trigger different error conditions
- The PUT example is simplified and doesn't include request body data
