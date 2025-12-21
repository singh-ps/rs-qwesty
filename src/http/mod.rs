mod client;
mod reqwest;

pub use client::Client;
pub use reqwest::delete;
pub use reqwest::get;
pub use reqwest::put;
pub use reqwest::{post, post_empty};
