# rust-slack
[![Travis Build Status](https://img.shields.io/travis/frostly/rust-slack.svg)](https://travis-ci.org/frostly/rust-slack)
[![Documentation](https://img.shields.io/badge/docs-latest-C9893D.svg)](https://open.frostly.com/rust-slack)
[![Coverage Status](https://img.shields.io/coveralls/frostly/rust-slack.svg)](https://coveralls.io/github/frostly/rust-slack?branch=master)
[![crates.io](https://img.shields.io/crates/v/slack-hook.svg)](https://crates.io/crates/slack-hook)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE-APACHE)

A rust crate for sending messages to Slack via webhooks.

[Slack](https://slack.com/) is a messaging platform for team collaboration.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
slack-hook = "0.1"
```

Add the crate to your existing project:

```rust,no_run
extern crate slack_hook;
use slack_hook::{Slack, Payload, PayloadTemplate};

fn main() {
    let slack = Slack::new("https://hooks.slack.com/services/abc/123/45z");
    let p = Payload::new(PayloadTemplate::Complete {
      text: Some("test message"),
      channel: Some("#testing"),
      username: Some("My Bot"),
      icon_url: None,
      icon_emoji: Some(":chart_with_upwards_trend:"),
      attachments: None,
      unfurl_links: Some(true),
      link_names: Some(false)
    });

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("ok"),
        Err(x) => println!("ERR: {:?}",x)
    }
}
```

## Attachments

To create a payload with just an attachment:

```rust
extern crate slack_hook;
use slack_hook::{Payload, PayloadTemplate, Attachment, AttachmentTemplate};

fn main() {
  let p = Payload::new(PayloadTemplate::Attachment {
    attachment: Attachment::new(AttachmentTemplate::Text {
      text: "my text",
      color: "#b13d41",
    }).unwrap(),
  });
}
```

# License

This library is distributed under similar terms to Rust: dual licensed under the MIT license and the Apache license (version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.
