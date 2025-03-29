use reqwest_middleware::reqwest;
use serde::de::DeserializeOwned;
use std::{error::Error, fmt::Debug};

pub async fn get<T>(url: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned + Debug,
{
    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.get(url).send().await?;
    response
        .json::<T>()
        .await
        .map_err(|error| Box::new(error).into())
}
