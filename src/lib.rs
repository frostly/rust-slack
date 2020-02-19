#![deny(missing_docs, missing_debug_implementations, trivial_casts, trivial_numeric_casts,
       unsafe_code, unstable_features, unused_import_braces, unused_qualifications, unused_results)]
#![cfg_attr(test, deny(warnings))]

//! Library to send messages to slack rooms
//! supports entire messaging API, including attachments and fields
//! also support for built-in colors as well as any hex colors

#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

pub use slack::{Slack, SlackLink, SlackUserLink, SlackText, SlackTextContent, SlackTime};
pub use payload::{Parse, Payload, PayloadBuilder};
pub use attachment::{Attachment, AttachmentBuilder, Field, Section, Action};
pub use crate::hex::{HexColor, SlackColor};
pub use error::{Error, Result};

#[macro_use] mod macros;
mod helper;
mod error;
mod hex;
mod payload;
mod attachment;
mod slack;
