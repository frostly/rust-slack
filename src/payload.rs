use {Attachment, Payload, SlackText, Parse, TryInto};
use helper::bool_to_u8;
use error::{Error, Result};
use url::Url;

/// `PayloadBuilder` is used to build a `Payload`
#[derive(Debug)]
pub struct PayloadBuilder {
    inner: Result<Payload>,
}

impl Default for PayloadBuilder {
    fn default() -> PayloadBuilder {
        PayloadBuilder { inner: Ok(Default::default()) }
    }
}

impl PayloadBuilder {
    /// Make a new `PayloadBuilder`
    pub fn new() -> PayloadBuilder {
        Default::default()
    }

    /// Set the text
    pub fn text<S: Into<SlackText>>(self, text: S) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.text = Some(text.into());
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Set the channel
    pub fn channel<S: Into<String>>(self, channel: S) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.channel = Some(channel.into());
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Set the username
    pub fn username<S: Into<String>>(self, username: S) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.username = Some(username.into());
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Set the icon_emoji
    pub fn icon_emoji<S: Into<String>>(self, icon_emoji: S) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.icon_emoji = Some(icon_emoji.into());
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    url_builder_fn! {
        /// Set the icon_url
        icon_url, PayloadBuilder
    }

    /// Set the attachments
    pub fn attachments(self, attachments: Vec<Attachment>) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.attachments = Some(attachments);
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// whether slack will try to fetch links and create an attachment
    /// https://api.slack.com/docs/unfurling
    pub fn unfurl_links(self, b: bool) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.unfurl_links = Some(b);
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Pass false to disable unfurling of media content
    pub fn unfurl_media(self, b: bool) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.unfurl_media = Some(b);
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Find and link channel names and usernames.
    // NOTE: The Slack API doesn't seem to actually require setting `link_names` to 1, any value
    // seems to work. However, to be faithful to their spec, we will keep the `bool_to_u8` fn
    // around.
    pub fn link_names(self, b: bool) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.link_names = Some(bool_to_u8(b));
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Change how messages are treated.
    pub fn parse(self, p: Parse) -> PayloadBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.parse = Some(p);
                PayloadBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Attempt to build the `Payload`
    pub fn build(self) -> Result<Payload> {
        self.inner
    }
}
