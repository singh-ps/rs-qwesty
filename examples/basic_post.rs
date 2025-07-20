use qwesty::http::post;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[allow(dead_code)]
struct CreateUserRequest {
    name: String,
    email: String,
    username: String,
}

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
    println!("Demonstrating POST request with qwesty...");

    // Create a request body for a new user
    let new_user = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        username: "johndoe".to_string(),
    };

    // Make the POST request to httpbin.org/post
    let response = post("https://httpbin.org/post", &new_user).await?;

    // Check response details
    println!("Response status: {}", response.status());
    println!("Is success: {}", response.is_success());

    if response.is_success() {
        let data = response.deserialize::<HttpBinPostResponse>().await?;
        println!("Successfully made POST request!");
        println!("Request came from: {}", data.origin);
        println!(
            "JSON data sent: {}",
            serde_json::to_string_pretty(&data.json)?
        );
    } else {
        println!("POST request failed with status: {}", response.status());
    }

    Ok(())
}
