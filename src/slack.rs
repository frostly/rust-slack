use std::fmt;
use curl::http;
use std::str;
use rustc_serialize::{json, Encodable, Encoder};
use types::{SlackResult, ErrSlackResp};
use payload::{Payload};

pub struct Slack {
    incoming_url: String,
}

impl Slack {
    pub fn new(url: &str) -> Slack {
        Slack {incoming_url: url.to_string()}
    }

    pub fn send(&self, payload: &Payload) -> SlackResult<()> {
        debug!("sending payload, {:?}", payload);
        debug!("JSON payload, {:?}", &json::encode(payload));
        let resp = http::handle()
          .post(self.incoming_url.as_slice(), &json::encode(payload))
          .exec().unwrap();
        debug!("slack response, {}", resp);

        let body = try!(str::from_utf8(resp.get_body()));

        match body {
            "ok" => Ok(()),
            x => fail!((ErrSlackResp, x)),
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

impl Encodable for SlackText {
  fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
      let text = format!("{:?}", &self);
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
        write!(f , "<{}|{:?}>" , self.url , self.text)
    }
}

impl Encodable for SlackLink {
  fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
      let text = format!("{:?}", &self);
      encoder.emit_str(text.as_slice())
  }
}

#[cfg(test)]
mod test {
    use test::Bencher;
    use slack::{Slack, SlackLink, SlackText};
    use payload::{Payload, PayloadTemplate};
    use attachment::{Attachment, AttachmentTemplate, Field};
    use rustc_serialize::{json};

    #[test]
    fn slack_incoming_url_test() {
        let s = Slack::new("https://hooks.slack.com/services/abc/123/45z");
        assert_eq!(s.incoming_url, "https://hooks.slack.com/services/abc/123/45z".to_string());
    }

    #[test]
    fn slack_text_test() {
        let s = SlackText::new("moo <&> moo");
        assert_eq!(format!("{:?}",s), "moo &lt;&amp;&gt; moo".to_string());
    }

    #[test]
    fn slack_link_test() {
        let s = SlackLink {
            text  : SlackText::new("moo <&> moo"),
            url   : "http://google.com".to_string(),
        };
        assert_eq!(format!("{:?}",s), "<http://google.com|moo &lt;&amp;&gt; moo>".to_string());
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
        }).unwrap()];

        let p = Payload::new(PayloadTemplate::Complete {
                text: Some("test message"),
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
