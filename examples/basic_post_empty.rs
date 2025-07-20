use qwesty::http::post_empty;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct HttpBinPostResponse {
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
    println!("Demonstrating empty POST request with qwesty...");

    // Make an empty POST request (no body)
    let response = post_empty("https://httpbin.org/post").await?;

    // Check response details
    println!("Response status: {}", response.status());
    println!("Is success: {}", response.is_success());

    if response.is_success() {
        let data = response.deserialize::<HttpBinPostResponse>().await?;
        println!("Successfully made empty POST request!");
        println!("Request came from: {}", data.origin);
        println!("No data was sent (empty body): '{}'", data.data);
        println!("JSON field is null: {}", data.json);
    } else {
        println!(
            "Empty POST request failed with status: {}",
            response.status()
        );
    }

    Ok(())
}
