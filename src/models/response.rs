use std::error::Error;

use crate::models::{ErrorMessage, HttpError};
use reqwest_middleware::reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

/// Represents an HTTP response.
/// This struct wraps the `reqwest::Response` and provides methods to access the response data.
pub struct HttpResponse {
    response: Response,
}

impl HttpResponse {
    pub fn new(response: Response) -> Self {
        Self { response }
    }

    pub fn is_success(&self) -> bool {
        self.response.status().is_success()
    }

    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    async fn json<T: DeserializeOwned>(self) -> Result<T, HttpError> {
        self.response.json().await.map_err(|e| {
            let mut msg = format!("Error deserializing: {}", e);

            let mut source = e.source();
            while let Some(err) = source {
                if let Some(serde_err) = err.downcast_ref::<serde_json::Error>() {
                    msg.push_str(&format!(": {}", serde_err));
                    break;
                }
                source = err.source();
            }

            HttpError::DeSerError(msg)
        })
    }

    pub async fn deserialize<T: DeserializeOwned>(self) -> Result<T, HttpError> {
        if self.is_success() {
            Ok(self.json().await?)
        } else {
            let err_response_res: Result<ErrorMessage, HttpError> = self.json().await;
            match err_response_res {
                Ok(error_response) => {
                    let api_error = HttpError::RequestFailed(error_response.message);
                    Err(api_error)
                }
                Err(e) => {
                    let msg = format!("Error deserializing response errors: {}", e);
                    Err(HttpError::DeSerError(msg))
                }
            }
        }
    }
}
