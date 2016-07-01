use slack::SlackText;
use attachment::Attachment;
use helper::{opt_str_to_slacktext, opt_str_to_string, opt_bool_to_u8};

/// Payload to send to slack
/// https://api.slack.com/incoming-webhooks
/// https://api.slack.com/methods/chat.postMessage
#[derive(RustcEncodable, Debug)]
pub struct Payload {
    /// text to send
    /// despite `text` stated as required, it does not seem to be
    pub text: Option<SlackText>,
    /// channel to send payload to
    /// note: if not provided, this will default to channel
    /// setup in slack
    pub channel: Option<String>,
    /// username override
    pub username: Option<String>,
    /// specific url for icon
    pub icon_url: Option<String>,
    /// emjoi for icon
    /// https://api.slack.com/methods/emoji.list
    pub icon_emoji: Option<String>,
    /// attachments to send
    pub attachments: Option<Vec<Attachment>>,
    /// whether slack will try to fetch links and create an attachment
    /// https://api.slack.com/docs/unfurling
    pub unfurl_links: Option<u8>,
    /// find and link channel names and usernames
    pub link_names: Option<u8>,
}

/// Templates to support common payload use cases
#[derive(Debug)]
pub enum PayloadTemplate<'a> {
    /// Specify the entire payload
    Complete {
        /// Text to send
        text: Option<&'a str>,
        /// Channel to send payload to
        /// note: if not provided, this will default to channel
        /// setup in slack
        channel: Option<&'a str>,
        /// Username override
        username: Option<&'a str>,
        /// Specific url for icon
        icon_url: Option<&'a str>,
        /// Emjoi for icon
        /// https://api.slack.com/methods/emoji.list
        icon_emoji: Option<&'a str>,
        /// Attachments to send
        attachments: Option<Vec<Attachment>>,
        /// Whether slack will try to fetch links and create an attachment
        /// https://api.slack.com/docs/unfurling
        unfurl_links: Option<bool>,
        /// Find and link channel names and usernames
        link_names: Option<bool>,
    },
    /// Simple payload with just a message
    Message {
        /// Text to send
        text: &'a str,
    },
    /// Attachment-only payload
    Attachment {
        /// Provide a single attachment
        attachment: Attachment,
    },
}

impl Payload {
    /// Construct a new Payload from a template
    pub fn new(t: PayloadTemplate) -> Payload {
        match t {
            PayloadTemplate::Complete { text,
                                        channel,
                                        username,
                                        icon_url,
                                        icon_emoji,
                                        attachments,
                                        unfurl_links,
                                        link_names } => {
                Payload {
                    text: opt_str_to_slacktext(&text),
                    channel: opt_str_to_string(&channel),
                    username: opt_str_to_string(&username),
                    icon_url: opt_str_to_string(&icon_url),
                    icon_emoji: opt_str_to_string(&icon_emoji),
                    attachments: attachments,
                    unfurl_links: opt_bool_to_u8(&unfurl_links),
                    link_names: opt_bool_to_u8(&link_names),
                }
            }
            PayloadTemplate::Message { text } => {
                Payload {
                    text: Some(SlackText::new(text)),
                    channel: None,
                    username: None,
                    icon_url: None,
                    icon_emoji: None,
                    attachments: None,
                    unfurl_links: None,
                    link_names: None,
                }
            }
            PayloadTemplate::Attachment { attachment } => {
                Payload {
                    text: None,
                    channel: None,
                    username: None,
                    icon_url: None,
                    icon_emoji: None,
                    attachments: Some(vec![attachment]),
                    unfurl_links: None,
                    link_names: None,
                }
            }
        }
    }
}
