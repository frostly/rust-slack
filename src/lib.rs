#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![cfg_attr(test, deny(warnings))]

//! Library to send messages to slack rooms
//! supports entire messaging API, including attachments and fields
//! also support for built-in colors as well as any hex colors

extern crate reqwest;

extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate hex as hexx;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url_serde;

pub use crate::attachment::{Action, Attachment, AttachmentBuilder, Field, Section};
pub use crate::error::{Error, Result};
pub use crate::hex::{HexColor, SlackColor};
pub use crate::payload::{Parse, Payload, PayloadBuilder};
pub use crate::slack::{Slack, SlackLink, SlackText, SlackTextContent, SlackTime, SlackUserLink};

#[macro_use]
mod macros;
mod attachment;
mod error;
mod helper;
mod hex;
mod payload;
mod slack;

/// Waiting to stabilize: https://github.com/rust-lang/rust/issues/33417
///
/// An attempted conversion that consumes `self`, which may or may not be expensive.
///
/// Library authors should not directly implement this trait, but should prefer implementing
/// the [`TryFrom`] trait, which offers greater flexibility and provides an equivalent `TryInto`
/// implementation for free, thanks to a blanket implementation in the standard library.
///
/// [`TryFrom`]: trait.TryFrom.html
pub trait TryInto<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Err;

    /// Performs the conversion.
    fn try_into(self) -> ::std::result::Result<T, Self::Err>;
}

/// Waiting to stabilize: https://github.com/rust-lang/rust/issues/33417
///
/// Attempt to construct `Self` via a conversion.
pub trait TryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Err;

    /// Performs the conversion.
    fn try_from(_: T) -> ::std::result::Result<Self, Self::Err>;
}

impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
{
    type Err = U::Err;

    fn try_into(self) -> ::std::result::Result<U, U::Err> {
        U::try_from(self)
    }
}

impl<'a> TryFrom<&'a str> for reqwest::Url {
    type Err = Error;

    fn try_from(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s.parse() {
            Ok(u) => Ok(u),
            Err(e) => Err(e.into()),
        }
    }
}
