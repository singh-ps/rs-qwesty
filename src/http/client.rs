use crate::models::{ClientConfig, HttpError};
use reqwest_middleware::{reqwest, ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ConfiguredClient {
    pub client: ClientWithMiddleware,
}

impl ConfiguredClient {
    /// Create a new `Client` with a `ClientConfig`
    pub fn try_new(config: &ClientConfig) -> Result<Self, HttpError> {
        let mut client_builder = reqwest::ClientBuilder::new();
        client_builder = client_builder.timeout(Duration::from_secs(config.timeout.into()));
        client_builder = client_builder.user_agent(&config.user_agent);
        client_builder = client_builder.default_headers((&config.headers).try_into()?);
        let client = client_builder.build().map_err(|e| {
            let msg = format!("Error buiding Client: {}", e);
            HttpError::ClientError(msg)
        })?;

        let mut retry_policy =
            ExponentialBackoff::builder().build_with_max_retries(config.retry_policy.retries);
        retry_policy.min_retry_interval = config.retry_policy.min_interval;
        retry_policy.max_retry_interval = config.retry_policy.max_interval;

        let retry_client = ClientBuilder::new(client.clone())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        Ok(Self {
            client: retry_client,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::get;
    use crate::models::{Headers, RetryPolicy};
    use mockito::Server;
    use std::time::Duration;

    #[tokio::test]
    async fn test_configured_client_custom_user_agent() {
        let mut server = Server::new_async().await;
        let custom_ua = "Custom-UA-123";

        let mock = server
            .mock("GET", "/test")
            .match_header("user-agent", custom_ua)
            .with_status(200)
            .create_async()
            .await;

        let config = ClientConfig::new(30, custom_ua.to_string(), Headers::default(), None);
        let client = ConfiguredClient::try_new(&config).unwrap();

        let url = format!("{}/test", server.url());
        let _ = get(&url, Some(client)).await;

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_configured_client_retries() {
        let mut server = Server::new_async().await;

        // Mock will expect exactly 3 requests (1 initial + 2 retries)
        let mock = server
            .mock("GET", "/retry")
            .with_status(500)
            .expect(3)
            .create_async()
            .await;

        let retry_policy = RetryPolicy {
            retries: 2,
            min_interval: Duration::from_millis(10),
            max_interval: Duration::from_millis(100),
        };

        let config = ClientConfig::new(
            30,
            "test-agent".to_string(),
            Headers::default(),
            Some(retry_policy),
        );
        let client = ConfiguredClient::try_new(&config).unwrap();

        let url = format!("{}/retry", server.url());
        let _ = get(&url, Some(client)).await;

        mock.assert_async().await;
    }
}
