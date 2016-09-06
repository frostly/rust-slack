use slack::SlackText;
use types::SlackResult;
use hex::{HexColor, HexColorT};
use helper::opt_str_to_slacktext;

/// Slack allows for attachments to be added to messages. See
/// https://api.slack.com/docs/attachments for more information.
#[derive(RustcEncodable, Debug)]
pub struct Attachment {
    /// Required text for attachment.
    /// Slack will use this text to display on devices that don't support markup.
    pub fallback: SlackText,
    /// Optional text for other devices, markup supported
    pub text: Option<SlackText>,
    /// Optional text that appears above attachment
    pub pretext: Option<SlackText>,
    /// Color of attachment
    pub color: HexColor,
    /// Fields are defined as an array, and hashes contained within it will be
    /// displayed in a table inside the message attachment.
    pub fields: Option<Vec<Field>>,
    /// Optional text that appears above the attachment block
    pub author_name: Option<SlackText>,
    /// Optional link to the author
    pub author_link: Option<SlackText>,
    /// Optional icon for the author
    pub author_icon: Option<SlackText>,
    /// Optional larger, bolder text above the main body
    pub title: Option<SlackText>,
    /// Optional URL to link to from the title
    pub title_link: Option<SlackText>,
    /// Optional URL to an image that will be displayed in the body
    pub image_url: Option<SlackText>,
    /// Optional URL to an image that will be displayed as a thumbnail to the
    /// right of the body
    pub thumb_url: Option<SlackText>,
    /// Optional text that will appear at the bottom of the attachment
    pub footer: Option<SlackText>,
    /// Optional URL to an image that will be displayed at the bottom of the
    /// attachment
    pub footer_icon: Option<SlackText>,
}

/// Attachment template to simplify constructing attachments
/// for common use cases.
#[derive(Debug)]
pub enum AttachmentTemplate<'a> {
    /// Specify all attributes of attachment
    Complete {
        /// Required text for attachment.
        /// Slack will use this text to display on devices that don't support markup.
        fallback: &'a str,
        /// Optional primary text of attachment
        text: Option<&'a str>,
        /// Optional text that appears above attachment
        pretext: Option<&'a str>,
        /// Color string can be any hex code starting with #
        color: &'a str,
        /// Fields are defined as an array, and hashes contained within it will
        /// be displayed in a table inside the message attachment.
        fields: Option<Vec<Field>>,
        /// Optional text that appears above the attachment block
        author_name: Option<&'a str>,
        /// Optional link to the author
        author_link: Option<&'a str>,
        /// Optional icon for the author
        author_icon: Option<&'a str>,
        /// Optional larger, bolder text above the main body
        title: Option<&'a str>,
        /// Optional URL to link to from the title
        title_link: Option<&'a str>,
        /// Optional URL to an image that will be displayed in the body
        image_url: Option<&'a str>,
        /// Optional URL to an image that will be displayed as a thumbnail to the
        /// right of the body
        thumb_url: Option<&'a str>,
        /// Optional text that will appear at the bottom of the attachment
        footer: Option<&'a str>,
        /// Optional URL to an image that will be displayed at the bottom of the
        /// attachment
        footer_icon: Option<&'a str>,
    },
    /// Provide only text and color for attachment
    /// other values will be defaulted
    Text {
        /// Text to send
        text: &'a str,
        /// Color string can be any hex code starting with #
        color: &'a str,
    },
}

impl Attachment {
    /// Construct new attachment based on template provided
    pub fn new(t: AttachmentTemplate) -> SlackResult<Attachment> {
        match t {
            AttachmentTemplate::Complete {
                fallback, text, pretext, color, fields, author_name,
                author_link, author_icon, title, title_link, image_url,
                thumb_url, footer, footer_icon
            } => {
                let c = try!(HexColorT::new(color));
                Ok(Attachment {
                    fallback: SlackText::new(fallback),
                    text: opt_str_to_slacktext(&text),
                    pretext: opt_str_to_slacktext(&pretext),
                    color: c,
                    fields: fields,
                    author_name: opt_str_to_slacktext(&author_name),
                    author_link: opt_str_to_slacktext(&author_link),
                    author_icon: opt_str_to_slacktext(&author_icon),
                    title: opt_str_to_slacktext(&title),
                    title_link: opt_str_to_slacktext(&title_link),
                    image_url: opt_str_to_slacktext(&image_url),
                    thumb_url: opt_str_to_slacktext(&thumb_url),
                    footer: opt_str_to_slacktext(&footer),
                    footer_icon: opt_str_to_slacktext(&footer_icon),
                })
            }
            AttachmentTemplate::Text { text, color } => {
                let c = try!(HexColorT::new(color));
                Ok(Attachment {
                    fallback: SlackText::new(text),
                    text: Some(SlackText::new(text)),
                    pretext: None,
                    color: c,
                    fields: None,
                    author_name: None,
                    author_link: None,
                    author_icon: None,
                    title: None,
                    title_link: None,
                    image_url: None,
                    thumb_url: None,
                    footer: None,
                    footer_icon: None,
                })
            }
        }
    }
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
    pub fn new(title: &str, value: &str, short: Option<bool>) -> Field {
        Field {
            title: title.to_owned(),
            value: SlackText::new(value),
            short: short,
        }
    }
}
