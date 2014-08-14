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
    pub text         : String,
    pub username     : Option<String>,
    pub icon_url     : Option<String>,
    pub icon_emoji   : Option<String>,
    pub attachments  : Option<Attachments>,
    pub unfurl_links : Option<u8>,
    pub link_names   : Option<u8>
}

#[deriving(Encodable, Show)]
pub struct Attachments {
    pub fallback : String,
    pub text     : Option<String>,
    pub pretext  : Option<String>,
    pub color    : String,
    pub fields   : Vec<Attachment>
}

#[deriving(Encodable, Show)]
pub struct Attachment {
    pub title : String,
    pub value : String,
    pub short : Option<bool>
}

#[test]
fn slack_test() {
    let s = Slack::new("hello.com".to_string(), "secret".to_string());
    assert_eq!(s.incoming_url, "https://hello.com.slack.com/services/hooks/incoming-webhook?token=secret".to_string());
}
