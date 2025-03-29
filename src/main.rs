use qwesty::{http::get, models::Assets};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = get::<Assets>("https://api.npoint.io/0e304b4f25305b0dd220").await?;
    println!("{:#?}", response);
    Ok(())
}
