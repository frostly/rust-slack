#![crate_name = "slack"]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate curl;
extern crate serialize;
#[cfg(test)] extern crate test;
pub use slack::{Slack, Payload, Attachment, Field, SlackText, SlackLink};

mod slack;
