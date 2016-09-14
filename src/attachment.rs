use error::{Error, Result};
use {Attachment, Field, HexColor, SlackText, TryInto, SlackTime};
use chrono::NaiveDateTime;
use url::Url;

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
    /// Optional small text used to display the author's name.
    pub fn author_name<S: Into<SlackText>>(self, author_name: S) -> AttachmentBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.author_name = Some(author_name.into());
                AttachmentBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    url_builder_fn! {
        /// Optional URL that will hyperlink the `author_name`.
        author_link, AttachmentBuilder
    }

    url_builder_fn! {
        /// Optional URL that displays a small 16x16px image to the left of the `author_name` text.
        author_icon, AttachmentBuilder
    }

    /// Optional larger, bolder text above the main body
    pub fn title<S: Into<SlackText>>(self, title: S) -> AttachmentBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.title = Some(title.into());
                AttachmentBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    url_builder_fn! {
        /// Optional URL to link to from the title
        title_link, AttachmentBuilder
    }

    url_builder_fn! {
        /// Optional URL to an image that will be displayed in the body
        image_url, AttachmentBuilder
    }

    url_builder_fn! {
        /// Optional URL to an image that will be displayed as a thumbnail to the right of the body
        thumb_url, AttachmentBuilder
    }

    /// Optional text that will appear at the bottom of the attachment
    pub fn footer<S: Into<SlackText>>(self, footer: S) -> AttachmentBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.footer = Some(footer.into());
                AttachmentBuilder { inner: Ok(inner) }
            }
            _ => self,
        }
    }

    url_builder_fn! {
        /// Optional URL to an image that will be displayed at the bottom of the attachment
        footer_icon, AttachmentBuilder
    }

    /// Optional timestamp to be displayed with the attachment
    pub fn ts(self, time: &NaiveDateTime) -> AttachmentBuilder {
        match self.inner {
            Ok(mut inner) => {
                inner.ts = Some(SlackTime::new(time));
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
