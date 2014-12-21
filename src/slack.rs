use std::fmt;
use curl::http;
use std::str;
use serialize::{json, Encodable, Encoder};

pub struct Slack {
    incoming_url: String,
}

impl Slack {
    pub fn new(url: &str) -> Slack {
        Slack {incoming_url: url.to_string()}
    }
    pub fn send(&self, payload: &Payload) -> Result<(), String> {
        debug!("sending payload, {}", payload);
        debug!("JSON payload, {}", &json::encode(payload));
        let resp = http::handle()
          .post(self.incoming_url.as_slice(), &json::encode(payload))
          .exec().unwrap();
        debug!("slack response, {}", resp);

        let body = str::from_utf8(resp.get_body());
        match body {
            Some("ok") => Ok(()),
            Some(x)    => Err(x.to_string()),
            None       => Err("no response given".to_string()),
        }
    }
}

#[deriving(Encodable, Show)]
pub struct Payload {
    pub text         : SlackText,
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
        text: &'a str,
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
                text         : SlackText(text.to_string()),
                channel      : opt_str_to_string(&channel),
                username     : opt_str_to_string(&username),
                icon_url     : opt_str_to_string(&icon_url),
                icon_emoji   : opt_str_to_string(&icon_emoji),
                attachments  : attachments,
                unfurl_links : opt_bool_to_u8(&unfurl_links),
                link_names   : opt_bool_to_u8(&link_names),
            },
            PayloadTemplate::Message { text } => Payload {
                text: SlackText(text.to_string()),
                channel: None,
                username: None,
                icon_url: None,
                icon_emoji: None,
                attachments: None,
                unfurl_links: None,
                link_names: None,
            },
        }
    }
}

fn opt_bool_to_u8(opt: &Option<bool>) -> Option<u8> {
    match opt {
        &Some(true) => Some(1u8),
        &Some(false) => Some(0u8),
        _ => None,
    }
}

fn opt_str_to_string(opt: &Option<&str>) -> Option<String> {
    match opt {
        &Some(x) => Some(x.to_string()),
        _ => None,
    }
}

#[deriving(Encodable, Show)]
pub struct Attachment {
    pub fallback : SlackText,
    pub text     : Option<SlackText>,
    pub pretext  : Option<SlackText>,
    pub color    : String,
    pub fields   : Option<Vec<Field>>,
}
pub enum AttachmentTemplate<'a> {
    Complete {
        fallback: &'a str,
        text: Option<&'a str>,
        pretext: Option<&'a str>,
        color: &'a str,
        fields: Option<Vec<Field>>,
    }
}
impl Attachment {
    pub fn new(t: AttachmentTemplate) -> Attachment {
        match t {
            AttachmentTemplate::Complete {
                fallback, text,
                pretext, color,
                fields } => Attachment {
                    fallback : SlackText(fallback.to_string()),
                    text     : opt_str_to_slacktext(&text),
                    pretext  : opt_str_to_slacktext(&pretext),
                    color    : color.to_string(),
                    fields   : fields,
            }
        }
    }
}

fn opt_str_to_slacktext(opt: &Option<&str>) -> Option<SlackText> {
    match opt {
        &Some(opt) => Some(SlackText(opt.to_string())),
        _         => None,
    }
}

#[deriving(Encodable, Show)]
pub struct Field {
    pub title : String,
    pub value : SlackText,
    pub short : Option<bool>,
}

impl Field {
    pub fn new(title: &str, value: &str, short: Option<bool>) -> Field {
        Field {
            title: title.to_string(),
            value: SlackText(value.to_string()),
            short: short,
        }
    }
}

pub struct SlackText(String);

impl SlackText {
    pub fn new(text: &str) -> SlackText {
        SlackText(text.to_string())
    }
}

impl fmt::Show for SlackText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f , "{}" , self.get_escaped_text())
    }
}

impl SlackText {
    fn get_escaped_text(&self) -> String {
        let SlackText(ref text) = *self;
        let mut escaped_text = String::new();
        for c in text.chars() {
            match c {
                '&' => escaped_text.push_str("&amp;"),
                '<' => escaped_text.push_str("&lt;"),
                '>' => escaped_text.push_str("&gt;"),
                _ => escaped_text.push(c),
            }
        }
        escaped_text
    }
}

impl <S: Encoder<E>, E> Encodable<S, E> for SlackText {
  fn encode(&self, encoder: &mut S) -> Result<(), E> {
      let text = format!("{}", &self);
      encoder.emit_str(text.as_slice())
  }
}

pub struct SlackLink {
    pub url  : String,
    pub text : SlackText,
}

impl SlackLink {
    pub fn new(url: &str, text: &str) -> SlackLink {
        return SlackLink {
            url  : url.to_string(),
            text : SlackText::new(text),
        }
    }
}

impl fmt::Show for SlackLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f , "<{}|{}>" , self.url , self.text)
    }
}

impl <S: Encoder<E>, E> Encodable<S, E> for SlackLink {
  fn encode(&self, encoder: &mut S) -> Result<(), E> {
      let text = format!("{}", &self);
      encoder.emit_str(text.as_slice())
  }
}

#[cfg(test)]
mod test {
    use test::Bencher;
    use slack::{Slack, SlackLink, SlackText, Payload, Attachment, PayloadTemplate, AttachmentTemplate, Field};
    use serialize::{json};

    #[test]
    fn slack_incoming_url_test() {
        let s = Slack::new("https://hooks.slack.com/services/abc/123/45z");
        assert_eq!(s.incoming_url, "https://hooks.slack.com/services/abc/123/45z".to_string());
    }

    #[test]
    fn slack_text_test() {
        let s = SlackText::new("moo <&> moo");
        assert_eq!(format!("{}",s), "moo &lt;&amp;&gt; moo".to_string());
    }

    #[test]
    fn slack_link_test() {
        let s = SlackLink {
            text  : SlackText::new("moo <&> moo"),
            url   : "http://google.com".to_string(),
        };
        assert_eq!(format!("{}",s), "<http://google.com|moo &lt;&amp;&gt; moo>".to_string());
    }

    #[test]
    fn json_slacklink_test() {
        let s = SlackLink {
            text  : SlackText::new("moo <&> moo"),
            url   : "http://google.com".to_string(),
        };
        assert_eq!(json::encode(&s).to_string(), "\"<http://google.com|moo &lt;&amp;&gt; moo>\"".to_string())
    }

    #[test]
    fn json_complete_payload_test() {
        let a = vec![Attachment::new(AttachmentTemplate::Complete {
            fallback: "fallback <&>",
            text: Some("text <&>"),
            pretext: None,
            color: "#6800e8",
            fields: Some(vec![Field::new("title", "value", None)]),
        })];

        let p = Payload::new(PayloadTemplate::Complete {
                text: "test message",
                channel: Some("#abc"),
                username: Some("Bot"),
                icon_url: None,
                icon_emoji: Some(":chart_with_upwards_trend:"),
                attachments: Some(a),
                unfurl_links: Some(false),
                link_names: Some(false),
            });

        assert_eq!(json::encode(&p).to_string(), r##"{"text":"test message","channel":"#abc","username":"Bot","icon_url":null,"icon_emoji":":chart_with_upwards_trend:","attachments":[{"fallback":"fallback &lt;&amp;&gt;","text":"text &lt;&amp;&gt;","pretext":null,"color":"#6800e8","fields":[{"title":"title","value":"value","short":null}]}],"unfurl_links":0,"link_names":0}"##.to_string())
    }

    #[test]
    fn json_message_payload_test() {
        let p = Payload::new(PayloadTemplate::Message {
                text: "test message",
            });

        assert_eq!(json::encode(&p).to_string(), r##"{"text":"test message","channel":null,"username":null,"icon_url":null,"icon_emoji":null,"attachments":null,"unfurl_links":null,"link_names":null}"##.to_string())
    }

    #[bench]
    fn bench_get_escaped_text(b: &mut Bencher) {
        let st = SlackText::new("moo <&> moo");
        b.iter(|| {
            st.get_escaped_text()
        })
    }
}
