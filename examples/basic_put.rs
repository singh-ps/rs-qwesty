use qwesty::models::HttpError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating PUT request with qwesty...");
    
    // Note: This is a simplified PUT example without request body
    // In a real scenario, PUT would typically include data to update
    match qwesty::http::put::<ApiResponse>("https://httpbin.org/put").await {
        Ok(response) => println!("✅ PUT request successful: {:?}", response),
        Err(HttpError::RequestFailed(msg)) => println!("❌ Request failed: {}", msg),
        Err(HttpError::DeSerError(msg)) => println!("❌ Deserialization failed: {}", msg),
        Err(HttpError::ClientError(msg)) => println!("❌ Client error: {}", msg),
    }
    
    println!("\nNote: This PUT example doesn't include a request body.");
    println!("In a real application, you would typically send data with PUT requests.");
    
    Ok(())
}
