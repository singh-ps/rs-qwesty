use crate::{
    http::reqwest::get_client,
    models::{HttpError, HttpResponse},
};
use serde::Serialize;

pub async fn post<T: Serialize>(url: &str, body: &T) -> Result<HttpResponse, HttpError> {
    let client = get_client().await?;

    let res = client
        .post(url)
        .json(body)
        .send()
        .await
        .map_err(|error| HttpError::RequestFailed(error.to_string()))?;

    Ok(HttpResponse::new(res))
}

pub async fn post_empty(url: &str) -> Result<HttpResponse, HttpError> {
    let client = get_client().await?;

    let res = client
        .post(url)
        .send()
        .await
        .map_err(|error| HttpError::RequestFailed(error.to_string()))?;

    Ok(HttpResponse::new(res))
}

#[cfg(test)]
mod tests {
    use super::{post, post_empty};
    use crate::models::HttpError;
    use mockito::Server;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Debug)]
    #[allow(dead_code)]
    struct CreateAssetRequest {
        name: String,
        version: String,
        url: String,
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
    async fn test_post_should_pass() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/endpoint")
            .with_status(201)
            .with_body(include_str!("fixtures/assets_response_good.json"))
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{}/endpoint", server_url);

        let request_body = CreateAssetRequest {
            name: "new-asset".to_string(),
            version: "1.0.0".to_string(),
            url: "https://example.com/asset".to_string(),
        };

        let response = post(url.as_str(), &request_body).await;

        mock.assert_async().await;
        let result = response.unwrap().deserialize::<Assets>().await.unwrap();
        assert_eq!(result.assets.len(), 4);
    }

    #[tokio::test]
    async fn test_post_missing_fields_should_fail() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/endpoint")
            .with_status(201)
            .with_body(include_str!("fixtures/assets_response_bad.json"))
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{}/endpoint", server_url);

        let request_body = CreateAssetRequest {
            name: "new-asset".to_string(),
            version: "1.0.0".to_string(),
            url: "https://example.com/asset".to_string(),
        };

        let response = post(url.as_str(), &request_body).await;

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
    async fn test_post_response_failed_should_fail() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/endpoint")
            .with_status(400)
            .with_body(r#"{"message": "Bad request"}"#)
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{}/endpoint", server_url);

        let request_body = CreateAssetRequest {
            name: "invalid-asset".to_string(),
            version: "1.0.0".to_string(),
            url: "https://example.com/asset".to_string(),
        };

        let response = post(url.as_str(), &request_body).await;

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

    #[tokio::test]
    async fn test_post_empty_should_pass() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/endpoint")
            .with_status(201)
            .with_body(include_str!("fixtures/assets_response_good.json"))
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{}/endpoint", server_url);

        let response = post_empty(url.as_str()).await;

        mock.assert_async().await;
        let result = response.unwrap().deserialize::<Assets>().await.unwrap();
        assert_eq!(result.assets.len(), 4);
    }
}
