#![crate_name = "slack"]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate curl;
extern crate serialize;

pub use slack::{Slack, Payload, Attachment, Attachments};

mod slack;
