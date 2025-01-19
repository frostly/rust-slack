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
        Ok(Slack {
            hook: hook.into_url()?,
            client: Client::new(),
        })
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
