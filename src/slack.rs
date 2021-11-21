use crate::error::{ErrorKind, Result};
use crate::Payload;
use chrono::NaiveDateTime;
use reqwest::{Client, Url};
use serde::{Serialize, Serializer};
use std::fmt;

/// Handles sending messages to slack
#[derive(Debug, Clone)]
pub struct Slack {
    hook: Url,
    client: Client,
}

impl Slack {
    /// Construct a new instance of slack for a specific incoming url endpoint.
    pub fn new<T: reqwest::IntoUrl>(hook: T) -> Result<Slack> {
        Ok(Slack {
            hook: hook.into_url()?,
            client: Client::new(),
        })
    }

    /// Send payload to slack service
    pub fn send(&self, payload: &Payload) -> Result<()> {
        let response = self.client.post(self.hook.clone()).json(payload).send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(ErrorKind::Slack(format!("HTTP error {}", response.status())).into())
        }
    }
}

/// Slack timestamp
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SlackTime(NaiveDateTime);

impl SlackTime {
    /// Construct a new `SlackTime`
    pub fn new(time: &NaiveDateTime) -> SlackTime {
        SlackTime(*time)
    }
}

impl Serialize for SlackTime {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}

/// Representation of any text sent through slack
/// the text must be processed to escape specific characters
#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub struct SlackText(String);

impl SlackText {
    /// Construct slack text with escaping
    /// Escape &, <, and > in any slack text
    /// https://api.slack.com/docs/formatting
    pub fn new<S: Into<String>>(text: S) -> SlackText {
        let s = text.into().chars().fold(String::new(), |mut s, c| {
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
#[derive(Debug, Clone, PartialEq)]
pub enum SlackTextContent {
    /// Text that will be escaped via slack api rules
    Text(SlackText),
    /// Link
    Link(SlackLink),
    /// User Link
    User(SlackUserLink),
}

impl<'a> From<&'a [SlackTextContent]> for SlackText {
    fn from(v: &[SlackTextContent]) -> SlackText {
        let st = v
            .iter()
            .map(|item| match *item {
                SlackTextContent::Text(ref s) => format!("{}", s),
                SlackTextContent::Link(ref link) => format!("{}", link),
                SlackTextContent::User(ref u) => format!("{}", u),
            })
            .collect::<Vec<String>>()
            .join(" ");
        SlackText::new_raw(st)
    }
}

impl fmt::Display for SlackText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Representation of a link sent in slack
#[derive(Debug, Clone, PartialEq)]
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

impl fmt::Display for SlackLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}|{}>", self.url, self.text)
    }
}

impl Serialize for SlackLink {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self)[..])
    }
}

/// Representation of a user id link sent in slack
///
/// Cannot do @UGUID|handle links using SlackLink in the future due to
/// https://api.slack.com/changelog/2017-09-the-one-about-usernames
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SlackUserLink {
    /// User ID (U1231232123) style
    pub uid: String,
}

impl SlackUserLink {
    /// Construct new `SlackUserLink` with a string slice
    pub fn new(uid: &str) -> SlackUserLink {
        SlackUserLink {
            uid: uid.to_owned(),
        }
    }
}

impl fmt::Display for SlackUserLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.uid)
    }
}

impl Serialize for SlackUserLink {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self)[..])
    }
}

#[cfg(test)]
mod test {
    use crate::slack::{Slack, SlackLink};
    use crate::{AttachmentBuilder, Field, Parse, PayloadBuilder, SlackText};
    use chrono::NaiveDateTime;
    #[cfg(feature = "unstable")]
    use test::Bencher;

    #[test]
    fn slack_incoming_url_test() {
        let s = Slack::new("https://hooks.slack.com/services/abc/123/45z").unwrap();
        assert_eq!(
            s.hook.to_string(),
            "https://hooks.slack.com/services/abc/123/45z".to_owned()
        );
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
        assert_eq!(
            format!("{}", s),
            "<http://google.com|moo &lt;&amp;&gt; moo>".to_owned()
        );
    }

    #[test]
    fn json_slacklink_test() {
        let s = SlackLink {
            text: SlackText::new("moo <&> moo"),
            url: "http://google.com".to_owned(),
        };
        assert_eq!(
            serde_json::to_string(&s).unwrap(),
            "\"<http://google.com|moo &lt;&amp;&gt; moo>\"".to_owned()
        )
    }

    #[test]
    fn json_complete_payload_test() {
        let a = vec![AttachmentBuilder::new("fallback <&>")
            .text("text <&>")
            .color("#6800e8")
            .fields(vec![Field::new("title", "value", None)])
            .title_link("https://title_link.com/")
            .ts(&NaiveDateTime::from_timestamp(123_456_789, 0))
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

        assert_eq!(serde_json::to_string(&p).unwrap(),
            r##"{"text":"test message","channel":"#abc","username":"Bot","icon_url":"https://example.com/","icon_emoji":":chart_with_upwards_trend:","attachments":[{"fallback":"fallback &lt;&amp;&gt;","text":"text &lt;&amp;&gt;","color":"#6800e8","fields":[{"title":"title","value":"value"}],"title_link":"https://title_link.com/","ts":123456789}],"unfurl_links":false,"link_names":1,"parse":"full"}"##.to_owned())
    }

    #[test]
    fn json_message_payload_test() {
        let p = PayloadBuilder::new().text("test message").build().unwrap();

        assert_eq!(
            serde_json::to_string(&p).unwrap(),
            r##"{"text":"test message"}"##.to_owned()
        )
    }

    #[test]
    fn slack_text_content_test() {
        use super::SlackTextContent;
        use super::SlackTextContent::{Link, Text};
        let message: Vec<SlackTextContent> = vec![
            Text("moo <&> moo".into()),
            Link(SlackLink::new("@USER", "M<E>")),
            Text("wow.".into()),
        ];
        let st: SlackText = SlackText::from(&message[..]);
        assert_eq!(
            format!("{}", st),
            "moo &lt;&amp;&gt; moo <@USER|M&lt;E&gt;> wow."
        );
    }
}
