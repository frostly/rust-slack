use curl::easy::Easy;
use std::str;
use error::{Result, Error};
use {Payload, SlackText, TryInto, serde_json};
use serde::{Serialize, Serializer};
use url::Url;

/// Handles sending messages to slack
#[derive(Debug)]
pub struct Slack {
    /// Url provided by slack interface for incoming webhook
    incoming_url: Url,
}

impl Slack {
    /// Construct a new instance of slack for a specific
    /// incoming url endopoint
    pub fn new<S: TryInto<Url, Err = Error>>(url: S) -> Result<Slack> {
        let url = try!(url.try_into());
        Ok(Slack { incoming_url: url })
    }

    /// Send payload to slack service
    pub fn send(&self, payload: &Payload) -> Result<()> {
        debug!("sending payload, {:?}", payload);
        let encoded = try!(serde_json::to_string(payload));
        debug!("JSON payload, {:?}", encoded);
        let mut easy = Easy::new();
        try!(easy.url(&self.incoming_url[..]));

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
            (_, x) => Err(Error::Slack(x.into())),
        }
    }
}

impl SlackText {
    /// Construct slack text with escaping
    /// Escape &, <, and > in any slack text
    /// https://api.slack.com/docs/formatting
    pub fn new<S: Into<String>>(text: S) -> SlackText {
        let s = text.into()
            .chars()
            .fold(String::new(), |mut s, c| {
                match c {
                    '&' => s.push_str("&amp;"),
                    '<' => s.push_str("&lt;"),
                    '>' => s.push_str("&gt;"),
                    _ => s.push(c),
                }
                s
            });
        SlackText(s)
    }

    fn new_raw<S: Into<String>>(text: S) -> SlackText {
        SlackText(text.into())
    }
}

impl<'a> From<&'a str> for SlackText {
    fn from(s: &'a str) -> SlackText {
        SlackText::new(String::from(s))
    }
}

impl<'a> From<String> for SlackText {
    fn from(s: String) -> SlackText {
        SlackText::new(s)
    }
}

/// Enum used for constructing a text field having both `SlackText`(s) and `SlackLink`(s). The
/// variants should be used together in a `Vec` on any function having a `Into<SlackText>` trait
/// bound. The combined text will be space-separated.
#[derive(Debug)]
pub enum SlackTextContent {
    /// Text that will be escaped via slack api rules
    Text(SlackText),
    /// Link
    Link(SlackLink),
}

impl<'a> From<&'a [SlackTextContent]> for SlackText {
    fn from(v: &[SlackTextContent]) -> SlackText {
        let st = v.iter()
            .map(|item| {
                match *item {
                    SlackTextContent::Text(ref s) => format!("{}", s),
                    SlackTextContent::Link(ref link) => format!("{}", link),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
        SlackText::new_raw(st)
    }
}

impl ::std::fmt::Display for SlackText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Representation of a link sent in slack
#[derive(Debug)]
pub struct SlackLink {
    /// URL for link.
    ///
    /// NOTE: this is NOT a `Url` type because some of the slack "urls", don't conform to standard
    /// url parsing scheme, which are enforced by the `url` crate.
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

impl ::std::fmt::Display for SlackLink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "<{}|{}>", self.url, self.text)
    }
}

impl Serialize for SlackLink {
    fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}", self)[..])
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "unstable")]
    use test::Bencher;
    use slack::{Slack, SlackLink};
    use {PayloadBuilder, AttachmentBuilder, Field, SlackText, Parse, serde_json};
    use chrono::NaiveDateTime;

    #[test]
    fn slack_incoming_url_test() {
        let s = Slack::new("https://hooks.slack.com/services/abc/123/45z").unwrap();
        assert_eq!(s.incoming_url[..],
                   "https://hooks.slack.com/services/abc/123/45z".to_owned());
    }

    #[test]
    fn slack_text_test() {
        let s = SlackText::new("moo <&> moo");
        assert_eq!(format!("{}", s), "moo &lt;&amp;&gt; moo".to_owned());
    }

    #[test]
    fn slack_link_test() {
        let s = SlackLink {
            text: SlackText::new("moo <&> moo"),
            url: "http://google.com".to_owned(),
        };
        assert_eq!(format!("{}", s),
                   "<http://google.com|moo &lt;&amp;&gt; moo>".to_owned());
    }

    #[test]
    fn json_slacklink_test() {
        let s = SlackLink {
            text: SlackText::new("moo <&> moo"),
            url: "http://google.com".to_owned(),
        };
        assert_eq!(serde_json::to_string(&s).unwrap().to_owned(),
                   "\"<http://google.com|moo &lt;&amp;&gt; moo>\"".to_owned())
    }

    #[test]
    fn json_complete_payload_test() {
        let a = vec![AttachmentBuilder::new("fallback <&>")
                         .text("text <&>")
                         .color("#6800e8")
                         .fields(vec![Field::new("title", "value", None)])
                         .title_link("https://title_link.com/")
                         .ts(&NaiveDateTime::from_timestamp(123456789, 0))
                         .build()
                         .unwrap()];

        let p = PayloadBuilder::new()
            .text("test message")
            .channel("#abc")
            .username("Bot")
            .icon_emoji(":chart_with_upwards_trend:")
            .icon_url("https://example.com")
            .attachments(a)
            .unfurl_links(false)
            .link_names(true)
            .parse(Parse::Full)
            .build()
            .unwrap();

        assert_eq!(serde_json::to_string(&p).unwrap().to_owned(), r##"{"text":"test message","channel":"#abc","username":"Bot","icon_url":"https://example.com/","icon_emoji":":chart_with_upwards_trend:","attachments":[{"fallback":"fallback &lt;&amp;&gt;","text":"text &lt;&amp;&gt;","color":"#6800e8","fields":[{"title":"title","value":"value"}],"title_link":"https://title_link.com/","ts":123456789}],"unfurl_links":false,"link_names":1,"parse":"full"}"##.to_owned())
    }

    #[test]
    fn json_message_payload_test() {
        let p = PayloadBuilder::new().text("test message").build().unwrap();

        assert_eq!(serde_json::to_string(&p).unwrap().to_owned(),
                   r##"{"text":"test message"}"##.to_owned())
    }

    #[test]
    fn slack_text_content_test() {
        use super::SlackTextContent;
        use super::SlackTextContent::{Link, Text};
        let message: Vec<SlackTextContent> = vec![Text("moo <&> moo".into()),
                                                  Link(SlackLink::new("@USER", "M<E>")),
                                                  Text("wow.".into())];
        let st: SlackText = SlackText::from(&message[..]);
        assert_eq!(format!("{}", st),
                   "moo &lt;&amp;&gt; moo <@USER|M&lt;E&gt;> wow.");
    }
}
