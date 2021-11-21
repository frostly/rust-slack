#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    rust_2018_idioms
)]

//! Library to send messages to slack rooms
//! supports entire messaging API, including attachments and fields
//! also support for built-in colors as well as any hex colors

#[macro_use]
extern crate error_chain;

pub use crate::attachment::{Action, Attachment, AttachmentBuilder, Field, Section};
pub use crate::error::{Error, Result};
pub use crate::hex::{HexColor, SlackColor};
pub use crate::payload::{Parse, Payload, PayloadBuilder};
pub use crate::slack::{Slack, SlackLink, SlackText, SlackTextContent, SlackTime, SlackUserLink};

#[macro_use]
mod macros;

mod attachment;
mod error;
mod hex;
mod payload;
mod slack;
