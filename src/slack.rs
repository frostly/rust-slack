use std::fmt;
use curl::http;
use std::str;
use serialize::{json, Encodable, Encoder};

pub struct Slack {
    incoming_url : String
}

impl Slack {
    pub fn new(domain: String, token: String) -> Slack {
        let url = format!("https://{}.slack.com/services/hooks/incoming-webhook?token={}", domain, token);
        Slack {incoming_url: url}
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
            None       => Err("no response given".to_string())
        }
    }
}

#[deriving(Encodable, Show)]
pub struct Payload<'a> {
    pub channel      : String,
    pub text         : SlackText,
    pub username     : Option<String>,
    pub icon_url     : Option<String>,
    pub icon_emoji   : Option<String>,
    pub attachments  : Option<&'a[Attachment]>,
    pub unfurl_links : Option<u8>,
    pub link_names   : Option<u8>
}

impl<'a> Payload<'a> {
    pub fn new(channel: String, text: String, username: Option<String>, icon_url: Option<String>, icon_emoji: Option<String>, attachments: Option<&'a[Attachment]>, unfurl_links: Option<u8>, link_names: Option<u8>) -> Payload {
        return Payload {
            channel      : channel,
            text         : SlackText(text),
            username     : username,
            icon_url     : icon_url,
            icon_emoji   : icon_emoji,
            attachments  : attachments,
            unfurl_links : unfurl_links,
            link_names   : link_names
        }
    }
}

#[deriving(Encodable, Show)]
pub struct Attachment {
    pub fallback : SlackText,
    pub text     : Option<SlackText>,
    pub pretext  : Option<SlackText>,
    pub color    : String,
    pub fields   : Option<Vec<Field>>
}

impl Attachment {
    pub fn new(fallback: String, text: Option<String>, pretext: Option<String>, color: String, fields: Option<Vec<Field>>) -> Attachment {
        return Attachment {
            fallback : SlackText(fallback),
            text     : some_slacktext(text),
            pretext  : some_slacktext(pretext),
            color    : color,
            fields   : fields
        }
    }
}

fn some_slacktext(opt: Option<String>) -> Option<SlackText> {
    return match opt {
        Some(opt) => Some(SlackText(opt)),
        _         => None
    }
}

#[deriving(Encodable, Show)]
pub struct Field {
    pub title : String,
    pub value : SlackText,
    pub short : Option<bool>
}

pub struct SlackText(String);

impl fmt::Show for SlackText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let SlackText(ref text) = *self;
        let mut re = regex!("&");
        let mut t2 = re.replace_all(text.as_slice(), "&amp;");
        re = regex!("<");
        t2 = re.replace_all(t2.as_slice(), "&lt;");
        re = regex!(">");
        t2 = re.replace_all(t2.as_slice(), "&gt;");
        write!(f , "{}" , t2)
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
    pub fn new(url: String, text: String) -> SlackLink {
        return SlackLink {
            url  : url,
            text : SlackText(text)
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

#[test]
fn slack_incoming_url_test() {
    let s = Slack::new("hello.com".to_string(), "secret".to_string());
    assert_eq!(s.incoming_url, "https://hello.com.slack.com/services/hooks/incoming-webhook?token=secret".to_string());
}

#[test]
fn slack_text_test() {
    let s = SlackText("moo <&> moo".to_string());
    assert_eq!(format!("{}",s), "moo &lt;&amp;&gt; moo".to_string());
}

#[test]
fn slack_link_test() {
    let s = SlackLink {
        text  : SlackText("moo <&> moo".to_string()),
        url   : "http://google.com".to_string()
    };
    assert_eq!(format!("{}",s), "<http://google.com|moo &lt;&amp;&gt; moo>".to_string());
}

#[test]
fn json_slacklink_test() {
    let s = SlackLink {
        text  : SlackText("moo <&> moo".to_string()),
        url   : "http://google.com".to_string()
    };
    assert_eq!(json::encode(&s).to_string(), "\"<http://google.com|moo &lt;&amp;&gt; moo>\"".to_string())
}

#[test]
fn json_attachment_test() {
    let a = [Attachment::new(
        "fallback <&>".to_string(),
        Some("text <&>".to_string()),
        None,
        "#6800e8".to_string(),
        None)];

    let p = Payload::new(
        "#abc".to_string(),
        "test message".to_string(),
        Some("Bot".to_string()),
        None,
        Some(":chart_with_upwards_trend:".to_string()),
        Some(a.as_slice()),
        Some(0),
        Some(0));

    assert_eq!(json::encode(&p).to_string(), r##"{"channel":"#abc","text":"test message","username":"Bot","icon_url":null,"icon_emoji":":chart_with_upwards_trend:","attachments":[{"fallback":"fallback &lt;&amp;&gt;","text":"text &lt;&amp;&gt;","pretext":null,"color":"#6800e8","fields":null}],"unfurl_links":0,"link_names":0}"##.to_string())
}
