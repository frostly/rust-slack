#![feature(core)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(test))]

//! Library to send messages to slack rooms
//! supports entire messaging API, including attachements and fields
//! also support for built-in colors as well as any hex colors

#[macro_use] extern crate log;
#[cfg(test)] extern crate test;

extern crate curl;
extern crate rustc_serialize;

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

mod helper;
mod macros;
mod types;
mod hex;
mod payload;
mod attachment;
mod slack;
