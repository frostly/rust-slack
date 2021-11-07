use error::{Error, Result};
use helper::bool_to_u8;
use reqwest::Url;
use serde::{Serialize, Serializer};
use {Attachment, SlackText, TryInto};

/// Payload to send to slack
/// https://api.slack.com/incoming-webhooks
/// https://api.slack.com/methods/chat.postMessage
#[derive(Serialize, Debug, Default)]
pub struct Payload {
    /// text to send
    /// despite `text` stated as required, it does not seem to be
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<SlackText>,
    /// channel to send payload to
    /// note: if not provided, this will default to channel
    /// setup in slack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// username override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// specific url for icon
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "::url_serde")]
    pub icon_url: Option<Url>,
    /// emjoi for icon
    /// https://api.slack.com/methods/emoji.list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    /// attachments to send
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// whether slack will try to fetch links and create an attachment
    /// https://api.slack.com/docs/unfurling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_media: Option<bool>,
    /// find and link channel names and usernames
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<u8>,
    /// Change how messages are treated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<Parse>,
}

/// Change how messages are treated.
#[derive(Debug)]
pub enum Parse {
    /// Full
    Full,
    /// None
    None,
}

impl Serialize for Parse {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let st = match *self {
            Parse::Full => "full",
            Parse::None => "none",
        };
        serializer.serialize_str(st)
    }
}
/// `PayloadBuilder` is used to build a `Payload`
#[derive(Debug)]
pub struct PayloadBuilder {
    inner: Result<Payload>,
}

impl Default for PayloadBuilder {
    fn default() -> PayloadBuilder {
        PayloadBuilder {
            inner: Ok(Default::default()),
        }
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
