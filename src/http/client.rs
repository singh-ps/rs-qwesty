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
