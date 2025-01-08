use std::{fmt, str::Utf8Error};

use hex::FromHexError;

/// An alias for a `Result` with a `slack_hook::Error`
pub type Result<T> = std::result::Result<T, Error>;

/// The all-encompassing error type for the `slack-hook` crate
#[derive(Debug)]
pub enum Error {
    /// slack service error
    Slack(String),
    /// Hex color parsing error
    HexColor(String),
    /// utf8 error, slack responses should be valid utf8
    Utf8(Utf8Error),
    /// `serde_json::Error`
    Serialize(serde_json::Error),
    /// `hex::FromHexError`
    FromHex(FromHexError),
    /// `reqwest::Error`
    Reqwest(reqwest::Error),
    /// `url::ParseError`
    Url(url::ParseError),
    /// `std::io::Error`
    Io(std::io::Error),
}

impl From<Utf8Error> for Error {
    fn from(utf8_err: Utf8Error) -> Self {
        Self::Utf8(utf8_err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(json_err: serde_json::Error) -> Self {
        Self::Serialize(json_err)
    }
}

impl From<FromHexError> for Error {
    fn from(hex_err: FromHexError) -> Self {
        Self::FromHex(hex_err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_err: reqwest::Error) -> Self {
        Self::Reqwest(reqwest_err)
    }
}

impl From<url::ParseError> for Error {
    fn from(url_err: url::ParseError) -> Self {
        Self::Url(url_err)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Self {
        Self::Io(io_err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Slack(err) => write!(f, "slack service error: {err}"),
            Self::HexColor(err) => write!(f, "hex color parsing error: {err}"),
            Self::Utf8(err) => err.fmt(f),
            Self::Serialize(err) => err.fmt(f),
            Self::FromHex(err) => err.fmt(f),
            Self::Reqwest(err) => err.fmt(f),
            Self::Url(err) => err.fmt(f),
            Self::Io(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
