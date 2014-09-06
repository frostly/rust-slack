#![crate_name = "slack"]
#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;
#[phase(plugin, link)] extern crate log;
extern crate curl;
extern crate serialize;
pub use slack::{Slack, Payload, Attachment, Field, SlackText, SlackLink};

mod slack;
