# slack-hook

[![CI](https://github.com/frostly/rust-slack/actions/workflows/basic.yml/badge.svg)](https://github.com/frostly/rust-slack/actions/workflows/basic.yml)
[![Documentation](https://img.shields.io/badge/docs-latest-C9893D.svg)](https://docs.rs/slack-hook/)
[![crates.io](https://img.shields.io/crates/v/slack-hook.svg)](https://crates.io/crates/slack-hook)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE-APACHE)

A rust crate for sending messages to Slack via webhooks.

[Slack](https://slack.com/) is a messaging platform for team collaboration.

Upgrading? See the [CHANGELOG](./CHANGELOG.md).

# Features

- **blocking**: Provides a synchronous "blocking" slack client
- **default-tls** _(enabled by default)_: Provides TLS support to connect over HTTPS
- **native-tls**: Enables TLS functionality provided by [`native-tls`](https://crates.io/crates/native-tls)
- **rustls-tls**: Enables TLS functionality provided by [`rustls`](https://crates.io/crates/rustls)

# Usage

Simply run this to add it to your `Cargo.toml`:

```console
cargo add slack-hook --features=blocking
```

and then start sending messages!

```rust,no_run
use slack_hook::{blocking::Slack, PayloadBuilder};

let slack = Slack::new("https://hooks.slack.com/services/abc/123/45z").unwrap();
let payload = PayloadBuilder::new()
    .text("test message")
    .channel("#testing")
    .username("My Bot")
    .icon_emoji(":chart_with_upwards_trend:")
    .build()
    .expect("we know this payload is valid");

match slack.send(&payload) {
    Ok(()) => println!("Message sent!"),
    Err(err) => eprintln!("Error: {err:?}")
}
```

## Attachments

To create a payload with just an attachment:

```rust
use slack_hook::{PayloadBuilder, AttachmentBuilder};

let attachment = AttachmentBuilder::new("my text")
    .color("#b13d41")
    .build()
    .unwrap();
let _payload = PayloadBuilder::new()
    .attachments(vec![attachment])
    .build()
    .unwrap();
```

## Text with Links

Slack messaging API permits you to send links within text. However, given the
different formatting rules, these text fragments need to be specified as
follows:

```rust
use slack_hook::{PayloadBuilder, SlackTextContent, SlackLink};

let text = [
    SlackTextContent::Text("Hello".into()),
    SlackTextContent::Link(SlackLink::new("https://google.com", "Google")),
    SlackTextContent::Text(", nice to know you.".into())
];
let _ = PayloadBuilder::new()
.text(text.as_slice())
.build()
.unwrap();
```

Sending this payload will display the following in slack (note: each element
of the `Vec` has been space-separated):

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Hello [Google](https://google.com), nice to know you.

This technique can be used for any function that has the `Into<SlackText>`
trait bound.

# License

This library is distributed under similar terms to Rust: dual licensed under
the MIT license and the Apache license (version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
