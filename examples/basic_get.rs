use qwesty::http::get;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct HttpBinResponse {
    args: serde_json::Value,
    headers: serde_json::Value,
    origin: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating GET request with qwesty...");

    // httpbin.org/get returns information about the GET request
    let response = get("https://httpbin.org/get").await?;

    // Check response details
    println!("Response status: {}", response.status());
    println!("Is success: {}", response.is_success());

    if response.is_success() {
        let data = response.deserialize::<HttpBinResponse>().await?;
        println!("Successfully made GET request!");
        println!("Request came from: {}", data.origin);
        println!("Request URL: {}", data.url);
        println!(
            "Headers received: {}",
            serde_json::to_string_pretty(&data.headers)?
        );
    } else {
        println!("GET request failed with status: {}", response.status());
    }

    Ok(())
}
