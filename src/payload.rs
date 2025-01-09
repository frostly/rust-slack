use crate::{Attachment, Result, SlackText};
use reqwest::Url;
use serde::{Serialize, Serializer};

/// Payload to send to slack
/// <https://api.slack.com/incoming-webhooks>
/// <https://api.slack.com/methods/chat.postMessage>
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
    pub icon_url: Option<Url>,
    /// emoji for icon
    /// <https://api.slack.com/methods/emoji.list>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    /// attachments to send
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// whether slack will try to fetch links and create an attachment
    /// <https://api.slack.com/docs/unfurling>
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
#[must_use]
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
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the text
    pub fn text<S: Into<SlackText>>(mut self, text: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.text = Some(text.into());
        }
        self
    }

    /// Set the channel
    pub fn channel<S: Into<String>>(mut self, channel: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.channel = Some(channel.into());
        }
        self
    }

    /// Set the username
    pub fn username<S: Into<String>>(mut self, username: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.username = Some(username.into());
        }
        self
    }

    /// Set the icon_emoji
    pub fn icon_emoji<S: Into<String>>(mut self, icon_emoji: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.icon_emoji = Some(icon_emoji.into());
        }
        self
    }

    url_builder_fn! {
        /// Set the icon_url
        icon_url, Self
    }

    /// Set the attachments
    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.attachments = Some(attachments);
        }
        self
    }

    /// whether slack will try to fetch links and create an attachment
    /// <https://api.slack.com/docs/unfurling>
    pub fn unfurl_links(mut self, b: bool) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.unfurl_links = Some(b);
        }
        self
    }

    /// Pass false to disable unfurling of media content
    pub fn unfurl_media(mut self, b: bool) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.unfurl_media = Some(b);
        }
        self
    }

    /// Find and link channel names and usernames.
    // NOTE: The Slack API doesn't seem to actually require setting `link_names` to 1, any value
    // seems to work. However, to be faithful to their spec we will stick to 0 and 1
    pub fn link_names(mut self, b: bool) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.link_names = Some(u8::from(b));
        }
        self
    }

    /// Change how messages are treated.
    pub fn parse(mut self, p: Parse) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.parse = Some(p);
        }
        self
    }

    /// Attempt to build the `Payload`
    pub fn build(self) -> Result<Payload> {
        self.inner
    }
}
