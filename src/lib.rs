#![crate_name = "slack"]
extern crate curl;
extern crate serialize;

pub use slack::{Slack, Payload, Attachment, Attachments};

mod slack;

// #[cfg(test)]
