#![deny(missing_docs,
        missing_debug_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications,
        unused_results)]
#![cfg_attr(all(test, feature = "unstable"), feature(test))] // add feature test when testing and unstable feature is provided
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(any(feature = "clippy", feature = "unstable"), allow(unstable_features))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))]

//! Library to send messages to slack rooms
//! supports entire messaging API, including attachements and fields
//! also support for built-in colors as well as any hex colors

#[macro_use] extern crate log;
#[cfg(all(test, feature="unstable"))] extern crate test; // needed for benchmarking

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
