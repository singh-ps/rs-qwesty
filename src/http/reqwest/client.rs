use crate::models::HttpError;
use reqwest_middleware::reqwest::{Client, ClientBuilder};

pub async fn get_client() -> Result<Client, HttpError> {
    ClientBuilder::new()
        .build()
        .map_err(|error| HttpError::ClientError(error.to_string()))
}
