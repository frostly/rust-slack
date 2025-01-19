// TODO(cosmic): switch from deny to warn (except for `unsafe_code`) and prune some of these
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
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Library to send messages to slack rooms
//! supports entire messaging API, including attachments and fields
//! also support for built-in colors as well as any hex colors

// Run doctests on the README
#[cfg_attr(feature = "blocking", doc = include_str!("../README.md"))]
#[cfg(doctest)]
pub struct ReadmeDoctests;

// TODO(cosmic): We probably want _some_ level of nesting instead of having everything in the root
pub use crate::attachment::{Action, Attachment, AttachmentBuilder, Field, Section};
pub use crate::error::{Error, Result};
pub use crate::hex::{HexColor, SlackColor};
pub use crate::payload::{Parse, Payload, PayloadBuilder};
pub use crate::slack::{Slack, SlackLink, SlackText, SlackTextContent, SlackTime, SlackUserLink};

#[macro_use]
mod macros;

mod attachment;
/// A blocking slack client
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking;
mod error;
mod hex;
mod payload;
mod slack;
