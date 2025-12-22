use super::headers::Headers;
use std::time::Duration;

const DEFAULT_TIMEOUT: u32 = 60;

#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub retries: u32,
    pub min_interval: Duration,
    pub max_interval: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            retries: 0,
            min_interval: Duration::from_secs(1),
            max_interval: Duration::from_secs(60 * 60),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub headers: Headers,
    pub retry_policy: RetryPolicy,
    pub timeout: u32,
    pub user_agent: String,
}

impl Default for ClientConfig {
    /// The default Client Configuration
    fn default() -> Self {
        Self {
            headers: Headers::default(),
            retry_policy: RetryPolicy::default(),
            user_agent: Headers::default_user_agent(),
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl ClientConfig {
    pub fn new(
        timeout: u32,
        user_agent: String,
        mut headers: Headers,
        opt_retry_policy: Option<RetryPolicy>,
    ) -> Self {
        headers.add_user_agent(&user_agent);
        Self {
            timeout,
            user_agent,
            headers,
            retry_policy: opt_retry_policy.unwrap_or_default(),
        }
    }
}
