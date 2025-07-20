use qwesty::models::HttpError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating error handling with qwesty...");

    // This will succeed (JSONPlaceholder API)
    match qwesty::http::get::<User>("https://jsonplaceholder.typicode.com/users/1").await {
        Ok(user) => println!("✅ Successfully fetched user: {:?}", user),
        Err(e) => println!("❌ Error fetching user: {:?}", e),
    }

    // This will fail (invalid URL)
    match qwesty::http::get::<User>("https://invalid-url-that-doesnt-exist.com/user").await {
        Ok(user) => println!("✅ Successfully fetched user: {:?}", user),
        Err(HttpError::RequestFailed(msg)) => println!("❌ Request failed: {}", msg),
        Err(HttpError::DeSerError(msg)) => println!("❌ Deserialization failed: {}", msg),
        Err(HttpError::ClientError(msg)) => println!("❌ Client error: {}", msg),
    }

    // This will fail (invalid JSON structure)
    match qwesty::http::get::<User>("https://httpbin.org/json").await {
        Ok(user) => println!("✅ Successfully fetched user: {:?}", user),
        Err(HttpError::RequestFailed(msg)) => println!("❌ Request failed: {}", msg),
        Err(HttpError::DeSerError(msg)) => println!("❌ Deserialization failed: {}", msg),
        Err(HttpError::ClientError(msg)) => println!("❌ Client error: {}", msg),
    }

    Ok(())
}
