use crate::{
    http::ConfiguredClient,
    models::{ClientConfig, HttpError},
};
use reqwest_middleware::ClientWithMiddleware;

/// Creates a new Client with middleware if configured client is None
pub fn get_client(
    configured_client: Option<ConfiguredClient>,
) -> Result<ClientWithMiddleware, HttpError> {
    match configured_client {
        Some(config_client) => Ok(config_client.client),

        None => {
            let config_client = ConfiguredClient::try_new(&ClientConfig::default())?;
            Ok(config_client.client)
        }
    }
}
