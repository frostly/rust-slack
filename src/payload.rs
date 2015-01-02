use rustc_serialize::{Encodable, Encoder};
use slack::{SlackText};
use attachment::Attachment;
include!("helper.rs");

#[deriving(RustcEncodable, Show)]
pub struct Payload {
    /// despite `text` stated as required, it does not seem to be
    pub text         : Option<SlackText>,
    pub channel      : Option<String>,
    pub username     : Option<String>,
    pub icon_url     : Option<String>,
    pub icon_emoji   : Option<String>,
    pub attachments  : Option<Vec<Attachment>>,
    pub unfurl_links : Option<u8>,
    pub link_names   : Option<u8>,
}

pub enum PayloadTemplate<'a> {
    Complete {
        text: Option<&'a str>,
        channel: Option<&'a str>,
        username: Option<&'a str>,
        icon_url: Option<&'a str>,
        icon_emoji: Option<&'a str>,
        attachments: Option<Vec<Attachment>>,
        unfurl_links: Option<bool>,
        link_names: Option<bool>,
    },
    Message {
        text: &'a str,
    },
    Attachment {
        attachment: Attachment,
    },
}

impl Payload {
    pub fn new(t: PayloadTemplate) -> Payload {
        match t {
            PayloadTemplate::Complete {
                text,
                channel,
                username,
                icon_url,
                icon_emoji,
                attachments,
                unfurl_links,
                link_names,
            } => Payload {
                text         : opt_str_to_slacktext(&text),
                channel      : opt_str_to_string(&channel),
                username     : opt_str_to_string(&username),
                icon_url     : opt_str_to_string(&icon_url),
                icon_emoji   : opt_str_to_string(&icon_emoji),
                attachments  : attachments,
                unfurl_links : opt_bool_to_u8(&unfurl_links),
                link_names   : opt_bool_to_u8(&link_names),
            },
            PayloadTemplate::Message { text } => Payload {
                text: Some(SlackText::new(text)),
                channel: None,
                username: None,
                icon_url: None,
                icon_emoji: None,
                attachments: None,
                unfurl_links: None,
                link_names: None,
            },
            PayloadTemplate::Attachment { attachment } => Payload {
                text: None,
                channel: None,
                username: None,
                icon_url: None,
                icon_emoji: None,
                attachments: Some(vec![attachment]),
                unfurl_links: None,
                link_names: None,
            },
        }
    }
}

