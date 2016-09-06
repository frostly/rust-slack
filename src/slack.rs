use std::fmt;
use curl::easy::Easy;
use std::str;
use rustc_serialize::{json, Encodable, Encoder};
use types::{SlackResult, ErrSlackResp};
use payload::Payload;

/// Handles sending messages to slack
#[derive(Debug)]
pub struct Slack {
    /// Url provided by slack interface for incoming webhook
    incoming_url: String,
}

impl Slack {
    /// Construct a new instance of slack for a specific
    /// incoming url endopoint
    pub fn new(url: &str) -> Slack {
        Slack { incoming_url: url.to_owned() }
    }

    /// Send payload to slack service
    pub fn send(&self, payload: &Payload) -> SlackResult<()> {
        debug!("sending payload, {:?}", payload);
        let encoded = try!(json::encode(payload));
        debug!("JSON payload, {:?}", encoded);
        let mut easy = Easy::new();
        let _ = easy.url(&self.incoming_url[..]);

        try!(easy.post(true));
        try!(easy.post_fields_copy(encoded.as_bytes()));
        let mut data = Vec::new();
        {
            let mut transfer = easy.transfer();
            try!(transfer.write_function(|dt| {
                data.extend_from_slice(dt);
                Ok(dt.len())
            }));
            try!(transfer.perform());
        }

        let resp = try!(easy.response_code());
        debug!("slack response, {}", resp);

        let body = try!(str::from_utf8(&data[..]));

        match (resp, body) {
            (200, _) => Ok(()),
            (_, x) => fail!((ErrSlackResp, x)),
        }
    }
}

/// Representation of any text sent through slack
/// the text must be processed to escape specific characters
pub struct SlackText(String);

impl SlackText {
    /// Construct slack text
    pub fn new(text: &str) -> SlackText {
        SlackText(text.to_owned())
    }
}

impl fmt::Debug for SlackText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_escaped_text())
    }
}

impl SlackText {
    /// Escape &, <, and > in any slack text
    /// https://api.slack.com/docs/formatting
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
        encoder.emit_str(&text)
    }
}

/// Representation of a link sent in slack
pub struct SlackLink {
    /// URL for link
    pub url: String,
    /// Anchor text for link
    pub text: SlackText,
}

impl SlackLink {
    /// Construct new SlackLink with string slices
    pub fn new(url: &str, text: &str) -> SlackLink {
        SlackLink {
            url: url.to_owned(),
            text: SlackText::new(text),
        }
    }
}

impl fmt::Debug for SlackLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}|{:?}>", self.url, self.text)
    }
}

impl Encodable for SlackLink {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        let text = format!("{:?}", &self);
        encoder.emit_str(&text)
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "unstable")]
    use test::Bencher;
    use slack::{Slack, SlackLink, SlackText};
    use payload::{Payload, PayloadTemplate};
    use attachment::{Attachment, AttachmentTemplate, Field};
    use rustc_serialize::json;

    #[test]
    fn slack_incoming_url_test() {
        let s = Slack::new("https://hooks.slack.com/services/abc/123/45z");
        assert_eq!(s.incoming_url,
                   "https://hooks.slack.com/services/abc/123/45z".to_owned());
    }

    #[test]
    fn slack_text_test() {
        let s = SlackText::new("moo <&> moo");
        assert_eq!(format!("{:?}", s), "moo &lt;&amp;&gt; moo".to_owned());
    }

    #[test]
    fn slack_link_test() {
        let s = SlackLink {
            text: SlackText::new("moo <&> moo"),
            url: "http://google.com".to_owned(),
        };
        assert_eq!(format!("{:?}", s),
                   "<http://google.com|moo &lt;&amp;&gt; moo>".to_owned());
    }

    #[test]
    fn json_slacklink_test() {
        let s = SlackLink {
            text: SlackText::new("moo <&> moo"),
            url: "http://google.com".to_owned(),
        };
        assert_eq!(json::encode(&s).unwrap().to_owned(),
                   "\"<http://google.com|moo &lt;&amp;&gt; moo>\"".to_owned())
    }

    #[test]
    fn json_complete_payload_test() {
        let a = vec![Attachment::new(AttachmentTemplate::Complete {
                         fallback: "fallback <&>",
                         text: Some("text <&>"),
                         pretext: None,
                         color: "#6800e8",
                         fields: Some(vec![Field::new("title", "value", None)]),
                         author_name: Some("Author"),
                         author_link: Some("https://author.com"),
                         author_icon: Some("https://author.com/icon"),
                         title: Some("Title"),
                         title_link: Some("https://title.com"),
                         image_url: Some("https://title.com/image"),
                         thumb_url: Some("https://title.com/thumb"),
                         footer: Some("Footer"),
                         footer_icon: Some("https://footer.com/icon"),
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

        assert_eq!(json::encode(&p).unwrap().to_owned(), r##"{"text":"test message","channel":"#abc","username":"Bot","icon_url":null,"icon_emoji":":chart_with_upwards_trend:","attachments":[{"fallback":"fallback &lt;&amp;&gt;","text":"text &lt;&amp;&gt;","pretext":null,"color":"#6800e8","fields":[{"title":"title","value":"value","short":null}],"author_name":"Author","author_link":"https://author.com","author_icon":"https://author.com/icon","title":"Title","title_link":"https://title.com","image_url":"https://title.com/image","thumb_url":"https://title.com/thumb","footer":"Footer","footer_icon":"https://footer.com/icon"}],"unfurl_links":0,"link_names":0}"##.to_owned())
    }

    #[test]
    fn json_message_payload_test() {
        let p = Payload::new(PayloadTemplate::Message { text: "test message" });

        assert_eq!(json::encode(&p).unwrap().to_owned(), r##"{"text":"test message","channel":null,"username":null,"icon_url":null,"icon_emoji":null,"attachments":null,"unfurl_links":null,"link_names":null}"##.to_owned())
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn bench_get_escaped_text(b: &mut Bencher) {
        let st = SlackText::new("moo <&> moo");
        b.iter(|| st.get_escaped_text())
    }
}
