#![crate_name = "slack"]
#![feature(macro_rules)]
#![feature(phase, globs)]
#[phase(plugin, link)] extern crate log;
extern crate curl;
extern crate "rustc-serialize" as rustc_serialize;
#[cfg(test)] extern crate test;
pub use slack::{Slack, Payload, Attachment, Field, SlackText, SlackLink, PayloadTemplate, AttachmentTemplate};


pub use types::{
    SlackError,
    SlackResult,
    ErrorKind,
};

mod macros;
mod types;

mod slack;
