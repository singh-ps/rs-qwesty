use qwesty::http::delete;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct HttpBinDeleteResponse {
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
    println!("Demonstrating DELETE request with qwesty...");

    // Make a DELETE request to httpbin.org/delete
    let response = delete("https://httpbin.org/delete").await?;

    // Check response details
    println!("Response status: {}", response.status());
    println!("Is success: {}", response.is_success());

    if response.is_success() {
        let data = response.deserialize::<HttpBinDeleteResponse>().await?;
        println!("Successfully made DELETE request!");
        println!("Request came from: {}", data.origin);
        println!("Request URL: {}", data.url);
    } else {
        println!("DELETE request failed with status: {}", response.status());
    }

    Ok(())
}
