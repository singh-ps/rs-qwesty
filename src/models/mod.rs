use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub url: String,
    pub hash: String,
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Assets {
    pub assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

use thiserror::Error;

#[derive(Error, Clone, Debug, serde::Deserialize, PartialEq)]
pub enum HttpError {
    #[error("Request failed: `{0}`")]
    RequestFailed(String),
    #[error("Unable to deserialize: `{0}`")]
    DeSerError(String),
    #[error("Unable to create client: `{0}`")]
    ClientError(String),
}
