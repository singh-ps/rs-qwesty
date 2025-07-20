use qwesty::http::put;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[allow(dead_code)]
struct UpdateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct HttpBinPutResponse {
    args: serde_json::Value,
    data: String,
    files: serde_json::Value,
    form: serde_json::Value,
    headers: serde_json::Value,
    json: serde_json::Value,
    origin: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating PUT request with qwesty...");

    // Create a request body
    let update_request = UpdateUserRequest {
        name: "Updated Name".to_string(),
        email: "updated@example.com".to_string(),
    };

    // Make the PUT request to httpbin.org/put
    let response = put("https://httpbin.org/put", &update_request).await?;

    // Check response details
    println!("Response status: {}", response.status());
    println!("Is success: {}", response.is_success());

    if response.is_success() {
        let data = response.deserialize::<HttpBinPutResponse>().await?;
        println!("Successfully made PUT request!");
        println!("Request came from: {}", data.origin);
        println!(
            "JSON data sent: {}",
            serde_json::to_string_pretty(&data.json)?
        );
    } else {
        println!("PUT request failed with status: {}", response.status());
    }

    Ok(())
}
