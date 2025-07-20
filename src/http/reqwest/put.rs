use crate::{
    http::reqwest::get_client,
    models::{HttpError, HttpResponse},
};
use serde::Serialize;

pub async fn put<T: Serialize>(url: &str, body: &T) -> Result<HttpResponse, HttpError> {
    let client = get_client().await?;

    let res = client
        .put(url)
        .json(body)
        .send()
        .await
        .map_err(|error| HttpError::RequestFailed(error.to_string()))?;

    Ok(HttpResponse::new(res))
}

#[cfg(test)]
mod tests {
    use super::put;
    use crate::models::HttpError;
    use mockito::Server;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Debug)]
    #[allow(dead_code)]
    struct UpdateAssetRequest {
        name: String,
        version: String,
    }

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
    async fn test_put_should_pass() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("PUT", "/endpoint")
            .with_status(200)
            .with_body(include_str!("fixtures/assets_response_good.json"))
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{server_url}/endpoint");

        let request_body = UpdateAssetRequest {
            name: "test-asset".to_string(),
            version: "1.0.0".to_string(),
        };

        let response = put(url.as_str(), &request_body).await;

        mock.assert_async().await;
        let result = response.unwrap().deserialize::<Assets>().await.unwrap();
        assert_eq!(result.assets.len(), 4);
    }

    #[tokio::test]
    async fn test_put_missing_fields_should_fail() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("PUT", "/endpoint")
            .with_status(200)
            .with_body(include_str!("fixtures/assets_response_bad.json"))
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{server_url}/endpoint");

        let request_body = UpdateAssetRequest {
            name: "test-asset".to_string(),
            version: "1.0.0".to_string(),
        };

        let response = put(url.as_str(), &request_body).await;

        mock.assert_async().await;
        assert!(response.is_ok());
        let deserialization_result = response.unwrap().deserialize::<Assets>().await;
        assert!(deserialization_result.is_err());
        // The exact error message may vary, so just check it's a DeSerError
        assert!(matches!(
            deserialization_result.err().unwrap(),
            HttpError::DeSerError(_)
        ));
    }

    #[tokio::test]
    async fn test_put_response_failed_should_fail() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("PUT", "/endpoint")
            .with_status(500)
            .with_body(r#"{"message": "Internal server error"}"#)
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{server_url}/endpoint");

        let request_body = UpdateAssetRequest {
            name: "test-asset".to_string(),
            version: "1.0.0".to_string(),
        };

        let response = put(url.as_str(), &request_body).await;

        mock.assert_async().await;
        assert!(response.is_ok());
        let deserialization_result = response.unwrap().deserialize::<Assets>().await;
        assert!(deserialization_result.is_err());
        // Should be a RequestFailed error because the status is not success
        assert!(matches!(
            deserialization_result.err().unwrap(),
            HttpError::RequestFailed(_)
        ));
    }
}
