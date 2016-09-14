#![deny(missing_docs,
        missing_debug_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications,
        unused_results)]
// add feature test when testing and unstable feature is provided
#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(any(feature = "clippy", feature = "unstable"), allow(unstable_features))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))]

//! Library to send messages to slack rooms
//! supports entire messaging API, including attachments and fields
//! also support for built-in colors as well as any hex colors

#[macro_use]
extern crate log;
#[cfg(all(test, feature="unstable"))]
extern crate test; // needed for benchmarking

extern crate curl;
#[macro_use]
extern crate quick_error;
extern crate serde;
extern crate serde_json;
extern crate hex as hexx;
pub extern crate url;
pub extern crate chrono;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

pub use slack::{Slack, SlackTextContent, SlackLink};
pub use payload::PayloadBuilder;
pub use attachment::AttachmentBuilder;
pub use hex::SlackColor;
pub use error::{Error, Result};
use url::Url;

#[macro_use]
mod macros;
mod helper;
mod error;
mod hex;
mod payload;
mod attachment;
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
    fn try_from(T) -> ::std::result::Result<Self, Self::Err>;
}

impl<T, U> TryInto<U> for T
    where U: TryFrom<T>
{
    type Err = U::Err;

    fn try_into(self) -> ::std::result::Result<U, U::Err> {
        U::try_from(self)
    }
}

impl<'a> TryFrom<&'a str> for Url {
    type Err = Error;
    fn try_from(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match Url::parse(s) {
            Ok(u) => Ok(u),
            Err(e) => Err(e.into()),
        }
    }
}
