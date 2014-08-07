use curl::http;
use serialize::{json, Encodable};

pub struct Slack {
    pub domain : &'static str,
    pub token  : &'static str
}

impl Slack {
    pub fn send(&self, payload: Payload) {
        let url = format!("https://{}.slack.com/services/hooks/incoming-webhook?token={}",self.domain, self.token);
        println!("url = {}", url);
        println!("sending payload, {}", payload);
        let resp = http::handle()
          .post(url, &json::encode(&payload))
          .exec().unwrap();
        println!("{}",resp);
    }
}

#[deriving(Encodable, Show)]
pub struct Payload {
    pub channel      : &'static str,
    pub text         : &'static str,
    pub username     : Option<&'static str>,
    pub icon_url     : Option<&'static str>,
    pub icon_emoji   : Option<&'static str>,
    pub attachments  : Option<Attachments>,
    pub unfurl_links : Option<u8>,
    pub link_names   : Option<u8>
}

#[deriving(Encodable, Show)]
pub struct Attachments {
    pub fallback : &'static str,
    pub text     : Option<&'static str>,
    pub pretext  : Option<&'static str>,
    pub color    : &'static str,
    pub fields   : Vec<Attachment>
}

#[deriving(Encodable, Show)]
pub struct Attachment {
    pub title : &'static str,
    pub value : &'static str,
    pub short : Option<bool>
}

