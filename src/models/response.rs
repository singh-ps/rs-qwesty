use crate::models::{ErrorMessage, HttpError};
use reqwest_middleware::reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use std::error::Error;

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
            let mut msg = format!("Error deserializing: {e}");

            let mut source = e.source();
            while let Some(err) = source {
                if let Some(serde_err) = err.downcast_ref::<serde_json::Error>() {
                    msg.push_str(&format!(": {serde_err}"));
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
            let status = self.status();
            // Failure case: Capture body as text to provide better error context
            let body_text = self.response.text().await.map_err(|e| {
                HttpError::RequestFailed(format!("Status {}: (Body unreadable: {})", status, e))
            })?;

            // 1. Try to parse as our standard ErrorMessage JSON
            if let Ok(error_message) = serde_json::from_str::<ErrorMessage>(&body_text) {
                Err(HttpError::RequestFailed(error_message.message))
            } else {
                // 2. Fallback: Return the raw text body (useful for 404 HTML, 502 Bad Gateway, etc.)
                Err(HttpError::RequestFailed(format!(
                    "Status {}: {}",
                    status, body_text
                )))
            }
        }
    }
}
