use curl::http;
use std::str;
use serialize::{json, Encodable};

pub struct Slack {
    pub domain : &str,
    pub token  : &str
}

impl Slack {
    pub fn send(&self, payload: &Payload) -> Result<(), String> {
        let url = format!("https://{}.slack.com/services/hooks/incoming-webhook?token={}", self.domain, self.token);
        debug!("sending payload, {}", payload);
        let resp = http::handle()
          .post(url, &json::encode(payload))
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
    pub channel      : &str,
    pub text         : &str,
    pub username     : Option<&str>,
    pub icon_url     : Option<&str>,
    pub icon_emoji   : Option<&str>,
    pub attachments  : Option<Attachments>,
    pub unfurl_links : Option<u8>,
    pub link_names   : Option<u8>
}

#[deriving(Encodable, Show)]
pub struct Attachments {
    pub fallback : &str,
    pub text     : Option<&str>,
    pub pretext  : Option<&str>,
    pub color    : &str,
    pub fields   : Vec<Attachment>
}

#[deriving(Encodable, Show)]
pub struct Attachment {
    pub title : &str,
    pub value : &str,
    pub short : Option<bool>
}
