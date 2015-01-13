use rustc_serialize::{Encodable, Encoder};
use slack::{SlackText};
use types::{SlackResult};
use hex::{HexColor, HexColorT};
use helper::{
    opt_str_to_slacktext,
};

#[derive(RustcEncodable, Show)]
pub struct Attachment {
    pub fallback : SlackText,
    pub text     : Option<SlackText>,
    pub pretext  : Option<SlackText>,
    pub color    : HexColor,
    pub fields   : Option<Vec<Field>>,
}
pub enum AttachmentTemplate<'a> {
    Complete {
        fallback: &'a str,
        text: Option<&'a str>,
        pretext: Option<&'a str>,
        color: &'a str,
        fields: Option<Vec<Field>>,
    },
    Text {
        text: &'a str,
        color: &'a str,
    },
}
impl Attachment {
    pub fn new(t: AttachmentTemplate) -> SlackResult<Attachment> {
        match t {
            AttachmentTemplate::Complete {
                fallback, text,
                pretext, color,
                fields
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

#[derive(RustcEncodable, Show)]
pub struct Field {
    pub title : String,
    pub value : SlackText,
    pub short : Option<bool>,
}

impl Field {
    pub fn new(title: &str, value: &str, short: Option<bool>) -> Field {
        Field {
            title: title.to_string(),
            value: SlackText::new(value),
            short: short,
        }
    }
}
