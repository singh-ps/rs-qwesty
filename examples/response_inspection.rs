use qwesty::http::get;
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
    println!("Demonstrating HttpResponse methods...");
    
    let response = get("https://jsonplaceholder.typicode.com/users/1").await?;
    
    // Check response details before deserializing
    println!("Response status: {}", response.status());
    println!("Is success: {}", response.is_success());
    
    if response.is_success() {
        let user = response.deserialize::<User>().await?;
        println!("Successfully fetched user: {:?}", user);
    } else {
        println!("Request failed with status: {}", response.status());
    }
    
    Ok(())
}
