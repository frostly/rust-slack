# rust-slack

A rust crate for sending messages to Slack via webhooks.

[Slack](https://slack.com/) is a messaging platform for team collaboration.

[![Build Status](https://travis-ci.org/frostly/rust-slack.png?branch=master)](https://travis-ci.org/frostly/rust-slack)

# Usage

Add the crate to your existing project:

```rust
extern crate slack;
```

Use the components you need:

```rust
use slack::{Slack, Payload, Attachment, Attachments};
```


Add your slack integration:

```rust
fn main() {
    let slack = Slack::new("https://hooks.slack.com/services/abc/123/45z".to_string());
    let p = Payload {
      channel: "#testing".to_string(),
      text: "test message".to_string(),
      username: Some("My Bot".to_string()),
      icon_url: None,
      icon_emoji: Some(":chart_with_upwards_trend:".to_string()),
      attachments: None,
      unfurl_links: Some(0),
      link_names: Some(1)
    };

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("ok"),
        Err(x) => println!("ERR: {}",x)
    }
}
```

# License

This library is distributed under similar terms to Rust: dual licensed under the MIT license and the Apache license (version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.
