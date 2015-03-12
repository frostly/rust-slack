use rustc_serialize::{Encodable, Encoder};
use slack::{SlackText};
use types::{SlackResult};
use hex::{HexColor, HexColorT};
use helper::{
    opt_str_to_slacktext,
};

/// slack allows for attachments to be added to messages
/// https://api.slack.com/docs/attachments
#[derive(RustcEncodable, Debug)]
pub struct Attachment {
    /// required text for attachment
    /// slack will use to display on devices that don't support markup
    pub fallback : SlackText,
    /// optional text for other devices, markup supported
    pub text     : Option<SlackText>,
    /// optional text that appears above attachment
    pub pretext  : Option<SlackText>,
    /// color of attachment
    pub color    : HexColor,
    /// Fields are defined as an array, and hashes contained within it will be
    /// displayed in a table inside the message attachment.
    pub fields   : Option<Vec<Field>>,
}

/// attachment template to simplify constructing attachments
/// for common use cases
pub enum AttachmentTemplate<'a> {
    /// specify all attributes of attachment
    Complete {
        /// required text for attachment
        /// slack will use to display on devices that don't support markup
        fallback: &'a str,
        /// optional primary text of attachment
        text: Option<&'a str>,
        /// optional text that appears above attachment
        pretext: Option<&'a str>,
        /// color string can be any hex code starting with #
        color: &'a str,
        /// Fields are defined as an array, and hashes contained within it will
        /// be displayed in a table inside the message attachment.
        fields: Option<Vec<Field>>,
    },
    /// provide only text and color for attachemnt
    /// other values will be defaulted
    Text {
        /// text to send
        text: &'a str,
        /// color string can be any hex code starting with #
        color: &'a str,
    },
}

impl Attachment {
    /// construct new attachment based on template provided
    pub fn new(t: AttachmentTemplate) -> SlackResult<Attachment> {
        match t {
            AttachmentTemplate::Complete {
                fallback,
                text,
                pretext,
                color,
                fields,
            } => {
                let c = try!(HexColorT::new(color));
                Ok(Attachment {
                    fallback : SlackText::new(fallback),
                    text     : opt_str_to_slacktext(&text),
                    pretext  : opt_str_to_slacktext(&pretext),
                    color    : c,
                    fields   : fields,
                })
            },
            AttachmentTemplate::Text {
                text, color
            } => {
                let c = try!(HexColorT::new(color));
                Ok(Attachment {
                    fallback: SlackText::new(text),
                    text: Some(SlackText::new(text)),
                    pretext: None,
                    color: c,
                    fields: None
                })
            },
        }
    }
}

/// Fields are defined as an array, and hashes contained within it will
/// be displayed in a table inside the message attachment.
#[derive(RustcEncodable, Debug)]
pub struct Field {
    /// Shown as a bold heading above the value text.
    /// It cannot contain markup and will be escaped for you.
    pub title : String,
    /// The text value of the field. It may contain standard message markup
    /// and must be escaped as normal. May be multi-line.
    pub value : SlackText,
    /// An optional flag indicating whether the value is short enough to be
    /// displayed side-by-side with other values.
    pub short : Option<bool>,
}

impl Field {
    /// Construct a new field
    pub fn new(title: &str, value: &str, short: Option<bool>) -> Field {
        Field {
            title: title.to_string(),
            value: SlackText::new(value),
            short: short,
        }
    }
}
