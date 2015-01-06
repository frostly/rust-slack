#![crate_name = "slackhook"]
#![feature(macro_rules)]
#![feature(phase, globs)]
#![feature(associated_types)]
#![feature(old_orphan_check)]
#[phase(plugin, link)] extern crate log;
extern crate curl;
extern crate "rustc-serialize" as rustc_serialize;
#[cfg(test)] extern crate test;

pub use slack::{
    Slack,
    SlackText,
    SlackLink
};
pub use payload::{
    Payload,
    PayloadTemplate
};
pub use attachment::{
    Attachment,
    AttachmentTemplate,
    Field
};
pub use types::{
    SlackError,
    SlackResult,
    ErrorKind,
};

pub use hex::{
    SlackColor,
    HexColor,
};

mod macros;
mod types;
mod hex;
mod payload;
mod attachment;
mod slack;
