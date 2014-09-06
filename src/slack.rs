use std::fmt;
use curl::http;
use std::str;
use serialize::{json, Encodable};

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
        let resp = http::handle()
          .post(self.incoming_url.as_slice(), &json::encode(payload))
          .exec().unwrap();
        debug!("slack response, {}", resp);

        let body = str::from_utf8(resp.get_body());
        match body {
            Some("ok") => Ok(()),
            Some(x) => Err(x.to_string()),
            None => Err("no response given".to_string())
        }
    }
}

#[deriving(Encodable, Show)]
pub struct Payload {
    pub channel      : String,
    pub text         : SlackText,
    pub username     : Option<String>,
    pub icon_url     : Option<String>,
    pub icon_emoji   : Option<String>,
    pub attachments  : Option<Attachment>,
    pub unfurl_links : Option<u8>,
    pub link_names   : Option<u8>
}

#[deriving(Encodable, Show)]
pub struct Attachment {
    pub fallback : SlackText,
    pub text     : Option<SlackText>,
    pub pretext  : Option<SlackText>,
    pub color    : String,
    pub fields   : Option<Vec<Field>>
}

#[deriving(Encodable, Show)]
pub struct Field {
    pub title : String,
    pub value : SlackText,
    pub short : Option<bool>
}

#[deriving(Encodable)]
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

#[deriving(Encodable)]
pub struct SlackLink {
    pub url: String,
    pub text: SlackText,
}

impl fmt::Show for SlackLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f , "<{}|{}>" , self.url , self.text)
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
        text: SlackText("moo <&> moo".to_string()),
        url: "http://google.com".to_string()
    };
    assert_eq!(format!("{}",s), "<http://google.com|moo &lt;&amp;&gt; moo>".to_string());
}
