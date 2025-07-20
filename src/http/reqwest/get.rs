use crate::{
    http::reqwest::{get_client, handle_response},
    models::HttpError,
};
use serde::de::DeserializeOwned;

pub async fn get<T>(url: &str) -> Result<T, HttpError>
where
    T: DeserializeOwned,
{
    let client = get_client().await?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| HttpError::RequestFailed(error.to_string()))?;

    handle_response(response).await
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
        match response {
            Ok(assets) => {
                assert_eq!(assets.assets.len(), 4);
            }
            Err(e) => {
                panic!("Expected success but got error: {:?}", e);
            }
        }
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
