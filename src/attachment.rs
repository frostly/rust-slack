use slack::SlackText;
use error::{Error, Result};
use hex::HexColor;
use TryInto;

/// Slack allows for attachments to be added to messages. See
/// https://api.slack.com/docs/attachments for more information.
#[derive(RustcEncodable, Debug, Default)]
pub struct Attachment {
    /// Required text for attachment.
    /// Slack will use this text to display on devices that don't support markup.
    pub fallback: SlackText,
    /// Optional text for other devices, markup supported
    pub text: Option<SlackText>,
    /// Optional text that appears above attachment
    pub pretext: Option<SlackText>,
    /// Optional color of attachment
    pub color: Option<HexColor>,
    /// Fields are defined as an array, and hashes contained within it will be
    /// displayed in a table inside the message attachment.
    pub fields: Option<Vec<Field>>,
}

/// Fields are defined as an array, and hashes contained within it will
/// be displayed in a table inside the message attachment.
#[derive(RustcEncodable, Debug)]
pub struct Field {
    /// Shown as a bold heading above the value text.
    /// It cannot contain markup and will be escaped for you.
    pub title: String,
    /// The text value of the field. It may contain standard message markup
    /// and must be escaped as normal. May be multi-line.
    pub value: SlackText,
    /// An optional flag indicating whether the value is short enough to be
    /// displayed side-by-side with other values.
    pub short: Option<bool>,
}

impl Field {
    /// Construct a new field
    pub fn new<S: Into<String>, ST: Into<SlackText>>(title: S,
                                                     value: ST,
                                                     short: Option<bool>)
                                                     -> Field {
        Field {
            title: title.into(),
            value: value.into(),
            short: short,
        }
    }
}

/// `AttachmentBuilder` is used to build a `Attachment`
#[derive(Debug)]
pub struct AttachmentBuilder {
    inner: Result<Attachment>,
}

impl AttachmentBuilder {
    /// Make a new `AttachmentBuilder`
    ///
    /// Fallback is the only required field which is a plain-text summary of the attachment.
    pub fn new<S: Into<SlackText>>(fallback: S) -> AttachmentBuilder {
        AttachmentBuilder {
            inner: Ok(Attachment { fallback: fallback.into(), ..Default::default() }),
        }
    }

    /// Optional text that appears within the attachment
    pub fn text<S: Into<SlackText>>(self, text: S) -> AttachmentBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.text = Some(text.into());
                AttachmentBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
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
    pub fn color<C: TryInto<HexColor, Err = Error>>(self, color: C) -> AttachmentBuilder {
        match self.inner {
            Ok(mut inner) => {
                match color.try_into() {
                    Ok(c) => {
                        inner.color = Some(c);
                        AttachmentBuilder { inner: Ok(inner) }
                    }
                    Err(e) => AttachmentBuilder { inner: Err(e) },
                }
            }
            _ => self,
        }
    }

    /// Optional text that appears above the attachment block
    pub fn pretext<S: Into<SlackText>>(self, pretext: S) -> AttachmentBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.pretext = Some(pretext.into());
                AttachmentBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Fields are defined as an array, and hashes contained within it will be
    /// displayed in a table inside the message attachment.
    pub fn fields(self, fields: Vec<Field>) -> AttachmentBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.fields = Some(fields);
                AttachmentBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    /// Attempt to build the `Attachment`
    pub fn build(self) -> Result<Attachment> {
        // set text to equal fallback if text wasn't specified
        match self.inner {
            Ok(mut inner) => {
                if inner.text.is_none() {
                    inner.text = Some(inner.fallback.clone())
                }
                Ok(inner)
            }
            _ => self.inner,

        }
    }
}
