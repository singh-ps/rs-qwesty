use crate::models::{ErrorMessage, HttpError};
use reqwest_middleware::reqwest;
use serde::de::DeserializeOwned;

pub async fn get<T>(url: &str) -> Result<T, HttpError>
where
    T: DeserializeOwned,
{
    let client = reqwest::ClientBuilder::new()
        .build()
        .map_err(|error| HttpError::ClientError(error.to_string()))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| HttpError::RequestFailed(error.to_string()))?;

    if response.status().is_success() {
        response
            .json::<T>()
            .await
            .map_err(|error| HttpError::DeSerError(error.to_string()))
    } else {
        let status = response.status().to_string();
        let error_message = response
            .json::<ErrorMessage>()
            .await
            .map_err(|error| HttpError::RequestFailed(format!("{}: {}", status, error)))?;

        println!("This wont be reached");
        Err(HttpError::RequestFailed(format!(
            "{}: {}",
            status, error_message.message
        )))
    }
}
