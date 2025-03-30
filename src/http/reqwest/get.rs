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

#[cfg(test)]
mod tests {

    use crate::models::HttpError;

    use super::get;
    use mockito::Server;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
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

    #[tokio::test]
    async fn test_get_should_pass() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/endpoint")
            .with_status(200)
            .with_body(include_str!("fixtures/assets_response_good.json"))
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{}{}", server_url, "/endpoint");
        let response = get::<Assets>(url.as_str()).await;

        mock.assert_async().await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().assets.len(), 4);
    }

    #[tokio::test]
    async fn test_missing_fields_in_json_should_fail() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/endpoint")
            .with_status(200)
            .with_body(include_str!("fixtures/assets_response_bad.json"))
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{}{}", server_url, "/endpoint");
        let response = get::<Assets>(url.as_str()).await;

        mock.assert_async().await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap(),
            HttpError::DeSerError("error decoding response body".to_string())
        );
    }

    #[tokio::test]
    async fn test_response_failed_should_fail() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/endpoint")
            .with_status(500)
            .with_body(include_str!("fixtures/assets_response_bad.json"))
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{}{}", server_url, "/endpoint");
        let response = get::<Assets>(url.as_str()).await;

        mock.assert_async().await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap(),
            HttpError::RequestFailed(
                "500 Internal Server Error: error decoding response body".to_string()
            )
        );
    }
}
