use crate::error::{Error, Result};
use crate::{HexColor, SlackText, SlackTime};
use chrono::NaiveDateTime;
use reqwest::Url;
use serde::Serialize;
use std::convert::TryInto;

/// Slack allows for attachments to be added to messages. See
/// <https://api.slack.com/docs/attachments> for more information.
#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub struct Attachment {
    /// Required text for attachment.
    /// Slack will use this text to display on devices that don't support markup.
    pub fallback: SlackText,
    /// Optional text for other devices, markup supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<SlackText>,
    /// Optional text that appears above attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretext: Option<SlackText>,
    /// Optional color of attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<HexColor>,
    /// Actions as array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// Fields are defined as an array, and hashes contained within it will be
    /// displayed in a table inside the message attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    /// Optional small text used to display the author's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<SlackText>,
    /// Optional URL that will hyperlink the `author_name` text mentioned above. Will only
    /// work if `author_name` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_link: Option<Url>,
    /// Optional URL that displays a small 16x16px image to the left of
    /// the `author_name` text. Will only work if `author_name` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_icon: Option<Url>,
    /// Optional larger, bolder text above the main body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<SlackText>,
    /// Optional URL to link to from the title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_link: Option<Url>,
    /// Optional URL to an image that will be displayed in the body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<Url>,
    /// Optional URL to an image that will be displayed as a thumbnail to the
    /// right of the body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<Url>,
    /// Optional text that will appear at the bottom of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<SlackText>,
    /// Optional URL to an image that will be displayed at the bottom of the
    /// attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_icon: Option<Url>,
    /// Optional timestamp to be displayed with the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<SlackTime>,
    /// Optional sections formatted as markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrkdwn_in: Option<Vec<Section>>,
    /// Optional callback_id for actions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<SlackText>,
}

/// Sections define parts of an attachment.
#[derive(Eq, PartialEq, Copy, Clone, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Section {
    /// The pretext section.
    Pretext,
    /// The text section.
    Text,
    /// The fields.
    Fields,
}

/// Actions are defined as an array, and values contained within it will
/// be displayed with the message.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Action {
    /// Action type, renamed to 'type'
    #[serde(rename = "type")]
    pub action_type: String,
    /// Text for action
    pub text: String,
    /// Name of action
    pub name: String,
    /// Action style, ie: primary, danger, etc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// Value of action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Action {
    /// Construct a new field
    pub fn new<S: Into<String>>(
        action_type: S,
        text: S,
        name: S,
        style: Option<String>,
        value: Option<String>,
    ) -> Action {
        Action {
            action_type: action_type.into(),
            text: text.into(),
            name: name.into(),
            style,
            value,
        }
    }
}

/// Fields are defined as an array, and hashes contained within it will
/// be displayed in a table inside the message attachment.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Field {
    /// Shown as a bold heading above the value text.
    /// It cannot contain markup and will be escaped for you.
    pub title: String,
    /// The text value of the field. It may contain standard message markup
    /// and must be escaped as normal. May be multi-line.
    pub value: SlackText,
    /// An optional flag indicating whether the value is short enough to be
    /// displayed side-by-side with other values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<bool>,
}

impl Field {
    /// Construct a new field
    pub fn new<S: Into<String>, ST: Into<SlackText>>(
        title: S,
        value: ST,
        short: Option<bool>,
    ) -> Field {
        Field {
            title: title.into(),
            value: value.into(),
            short,
        }
    }
}

/// `AttachmentBuilder` is used to build a `Attachment`
#[derive(Debug)]
#[must_use]
pub struct AttachmentBuilder {
    inner: Result<Attachment>,
}

impl AttachmentBuilder {
    /// Make a new `AttachmentBuilder`
    ///
    /// Fallback is the only required field which is a plain-text summary of the attachment.
    // FIXME(cosmic): there's a bit of a miss-match where `fallback` is the only required field,
    // but an `Attachment` can still be constructed with purely default values :thinking_face:
    pub fn new<S: Into<SlackText>>(fallback: S) -> Self {
        Self {
            inner: Ok(Attachment {
                fallback: fallback.into(),
                ..Default::default()
            }),
        }
    }

    /// Optional text that appears within the attachment
    pub fn text<S: Into<SlackText>>(mut self, text: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.text = Some(text.into());
        }
        self
    }

    /// Set the color of the attachment
    ///
    /// The color can be one of:
    ///
    /// 1. `String`s: `good`, `warning`, `danger`
    /// 2. The built-in enums: `SlackColor::Good`, etc.
    /// 3. Any valid hex color code: e.g. `#b13d41` or `#000`.
    ///
    /// hex color codes will be checked to ensure a valid hex number is provided
    pub fn color<C: TryInto<HexColor, Error = Error>>(mut self, color: C) -> Self {
        if let Ok(inner) = &mut self.inner {
            match color.try_into() {
                Ok(c) => inner.color = Some(c),
                Err(err) => self.inner = Err(err),
            }
        }
        self
    }

    /// Optional text that appears above the attachment block
    pub fn pretext<S: Into<SlackText>>(mut self, pretext: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.pretext = Some(pretext.into());
        }
        self
    }
    /// Actions are defined as an array, and hashes contained within it will be
    /// displayed in a table inside the message attachment.
    pub fn actions(mut self, actions: Vec<Action>) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.actions = Some(actions);
        }
        self
    }
    /// Fields are defined as an array, and hashes contained within it will be
    /// displayed in a table inside the message attachment.
    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.fields = Some(fields);
        }
        self
    }
    /// Optional small text used to display the author's name.
    pub fn author_name<S: Into<SlackText>>(mut self, author_name: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.author_name = Some(author_name.into());
        }
        self
    }

    url_builder_fn! {
        /// Optional URL that will hyperlink the `author_name`.
        author_link, Self
    }

    url_builder_fn! {
        /// Optional URL that displays a small 16x16px image to the left of the `author_name` text.
        author_icon, Self
    }

    /// Optional larger, bolder text above the main body
    pub fn title<S: Into<SlackText>>(mut self, title: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.title = Some(title.into());
        }
        self
    }

    /// Optional larger, bolder text above the main body
    pub fn callback_id<S: Into<SlackText>>(mut self, callback_id: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.callback_id = Some(callback_id.into());
        }
        self
    }

    url_builder_fn! {
        /// Optional URL to link to from the title
        title_link, Self
    }

    url_builder_fn! {
        /// Optional URL to an image that will be displayed in the body
        image_url, Self
    }

    url_builder_fn! {
        /// Optional URL to an image that will be displayed as a thumbnail to the right of the body
        thumb_url, Self
    }

    /// Optional text that will appear at the bottom of the attachment
    pub fn footer<S: Into<SlackText>>(mut self, footer: S) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.footer = Some(footer.into());
        }
        self
    }

    url_builder_fn! {
        /// Optional URL to an image that will be displayed at the bottom of the attachment
        footer_icon, Self
    }

    /// Optional timestamp to be displayed with the attachment
    pub fn ts(mut self, time: &NaiveDateTime) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.ts = Some(SlackTime::new(time));
        }
        self
    }

    /// Optional sections formatted as markdown.
    pub fn markdown_in<'a, I: IntoIterator<Item = &'a Section>>(mut self, sections: I) -> Self {
        if let Ok(inner) = &mut self.inner {
            inner.mrkdwn_in = Some(sections.into_iter().cloned().collect());
        }
        self
    }

    /// Attempt to build the `Attachment`
    pub fn build(mut self) -> Result<Attachment> {
        // set text to equal fallback if text wasn't specified
        if let Ok(inner) = &mut self.inner {
            if inner.text.is_none() {
                inner.text = Some(inner.fallback.clone());
            }
        }
        self.inner
    }
}
