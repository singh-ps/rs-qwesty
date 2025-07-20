use reqwest_middleware::reqwest::Response;
use serde::de::DeserializeOwned;

use crate::models::{ErrorMessage, HttpError};

pub async fn handle_response<T>(response: Response) -> Result<T, HttpError>
where
    T: DeserializeOwned,
{
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

        Err(HttpError::RequestFailed(format!(
            "{}: {}",
            status, error_message.message
        )))
    }
}
