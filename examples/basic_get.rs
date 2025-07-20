use qwesty::http::get;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Asset {
    url: String,
    hash: String,
    name: String,
    version: String,
}

#[derive(Deserialize, Debug)]
struct Assets {
    assets: Vec<Asset>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Making a GET request to fetch assets...");

    let response = get("https://api.npoint.io/0e304b4f25305b0dd220").await?;
    let assets = response.deserialize::<Assets>().await?;

    println!("Successfully fetched {} assets:", assets.assets.len());
    for asset in assets.assets {
        println!("  - {} (v{}): {}", asset.name, asset.version, asset.url);
    }

    Ok(())
}
