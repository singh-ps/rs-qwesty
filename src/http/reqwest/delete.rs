use crate::{
    http::reqwest::get_client,
    models::{HttpError, HttpResponse},
};

pub async fn delete(url: &str) -> Result<HttpResponse, HttpError> {
    let client = get_client().await?;

    let res = client
        .delete(url)
        .send()
        .await
        .map_err(|error| HttpError::RequestFailed(error.to_string()))?;

    Ok(HttpResponse::new(res))
}

#[cfg(test)]
mod tests {
    use super::delete;
    use crate::models::HttpError;
    use mockito::Server;
    use serde::Deserialize;

    // Removed the unused DeleteRequest struct as it was not utilized in the code.

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    pub struct DeleteResponse {
        pub success: bool,
        pub message: String,
    }

    #[tokio::test]
    async fn test_delete_should_pass() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("DELETE", "/endpoint/1")
            .with_status(200)
            .with_body(r#"{"success": true, "message": "Resource deleted"}"#)
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{server_url}/endpoint/1");

        let response = delete(url.as_str()).await;

        mock.assert_async().await;
        let result = response
            .unwrap()
            .deserialize::<DeleteResponse>()
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.message, "Resource deleted");
    }

    #[tokio::test]
    async fn test_delete_response_failed_should_fail() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("DELETE", "/endpoint/999")
            .with_status(404)
            .with_body(r#"{"message": "Resource not found"}"#)
            .create_async()
            .await;

        let server_url = server.url();
        let url = format!("{server_url}/endpoint/999");

        let response = delete(url.as_str()).await;

        mock.assert_async().await;
        assert!(response.is_ok());
        let deserialization_result = response.unwrap().deserialize::<DeleteResponse>().await;
        assert!(deserialization_result.is_err());
        // Should be a RequestFailed error because the status is not success
        assert!(matches!(
            deserialization_result.err().unwrap(),
            HttpError::RequestFailed(_)
        ));
    }
}
