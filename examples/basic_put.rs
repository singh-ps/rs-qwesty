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
struct User {
    id: u32,
    name: String,
    email: String,
    // JSONPlaceholder doesn't return username in PUT responses sometimes
    // Making optional fields to handle partial responses
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating PUT request with qwesty...");
    
    // Create a request body
    let update_request = UpdateUserRequest {
        name: "Updated Name".to_string(),
        email: "updated@example.com".to_string(),
    };
    
    // Make the PUT request
    let response = put("https://jsonplaceholder.typicode.com/users/1", &update_request).await?;
    
    // Check response details
    println!("Response status: {}", response.status());
    println!("Is success: {}", response.is_success());
    
    if response.is_success() {
        let user = response.deserialize::<User>().await?;
        println!("Successfully updated user: {:?}", user);
    } else {
        println!("PUT request failed with status: {}", response.status());
    }
    
    Ok(())
}
