use crate::{Error, Payload, Result};

use reqwest::{blocking::Client, Url};

/// Handles sending messages to slack
#[derive(Debug, Clone)]
pub struct Slack {
    hook: Url,
    client: Client,
}

impl Slack {
    /// Construct a new instance of slack for a specific incoming url endpoint.
    pub fn new<T: reqwest::IntoUrl>(hook: T) -> Result<Slack> {
        Self::new_with_client(hook, Client::new())
    }

    /// The same as [`Slack::new()`], but with a custom [`reqwest::Client`]
    ///
    /// This allows for configuring custom proxies, DNS resolvers, etc.
    pub fn new_with_client<T: reqwest::IntoUrl>(hook: T, client: Client) -> Result<Self> {
        let hook = hook.into_url()?;
        Ok(Self { hook, client })
    }

    /// Send payload to slack service
    pub fn send(&self, payload: &Payload) -> Result<()> {
        let response = self.client.post(self.hook.clone()).json(payload).send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Slack(format!("HTTP error {}", response.status())))
        }
    }
}
