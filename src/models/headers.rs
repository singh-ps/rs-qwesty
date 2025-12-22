use reqwest_middleware::reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use rustc_version_runtime;
use std::collections::HashMap;
use std::env;

use crate::models::HttpError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Headers {
    pub headers: HashMap<String, String>,
}

impl Headers {
    /// Adds a request header
    pub fn add(&mut self, header_name: &str, header_value: &str) -> Option<String> {
        self.headers
            .insert(String::from(header_name), String::from(header_value))
    }

    /// Adds the User Agent header
    pub fn add_user_agent(&mut self, user_agent: &str) -> Option<String> {
        self.add("user-agent", user_agent)
    }

    /// Indicate if headers have a user-agent
    pub fn has_user_agent(&self) -> bool {
        self.headers.contains_key("user-agent")
    }

    pub(crate) fn default_user_agent() -> String {
        let version = rustc_version_runtime::version();
        let os: &str = env::consts::OS;
        format!("Qwesty Rust {version} on {os})")
    }
}

impl Default for Headers {
    /// The default set of request headers
    /// * Content-Type: application/json
    /// * accept: application/json
    /// * user-agent
    fn default() -> Self {
        let mut headers = HashMap::new();

        headers.insert(
            String::from("Content-Type"),
            String::from("application/json"),
        );
        headers.insert(String::from("accept"), String::from("application/json"));
        headers.insert(String::from("user-agent"), Headers::default_user_agent());

        Self { headers }
    }
}

impl TryFrom<&Headers> for HeaderMap {
    type Error = HttpError;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        let mut header_map = Self::new();
        for (k, v) in &headers.headers {
            let header_name = HeaderName::from_bytes(k.as_bytes()).map_err(|e| {
                let msg = format!("Error generating {} header name: {}", k, e);
                HttpError::HeaderError(msg)
            })?;
            let header_value = HeaderValue::from_bytes(v.as_bytes()).map_err(|e| {
                let msg = format!(
                    "Error generating {} header value for header {}: {}",
                    v, k, e
                );
                HttpError::HeaderError(msg)
            })?;
            header_map.insert(header_name, header_value);
        }

        Ok(header_map)
    }
}
