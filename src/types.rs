use std::{error, fmt, str};
use rustc_serialize::hex::FromHexError;
use rustc_serialize::json::EncoderError;
use curl;
use std::convert::From;

pub use self::ErrorKind::*;

/// Result alias to save typing
pub type SlackResult<T> = Result<T, SlackError>;

/// Different kinds of errors handled
#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    /// slack response failed
    ErrSlackResp,
    /// slack response should be in utf8
    ErrUtf8(str::Utf8Error),
    /// couldn't convert value to Hex
    ErrFromHex(FromHexError),
    /// failed other hex color validations for input
    ErrHexColor,
    /// failed to encode payload
    ErrEncoder(EncoderError),
    /// curl error
    ErrCurl(curl::ErrCode),
}

/// main Slack library error
#[derive(Debug)]
pub struct SlackError {
    /// kind of error
    pub kind: ErrorKind,
    /// description of error
    pub desc: String,
}

impl From<str::Utf8Error> for SlackError {
    fn from(err: str::Utf8Error) -> SlackError {
        SlackError {
            kind: ErrUtf8(err),
            desc: format!("{:?}", err),
        }
    }
}

impl From<EncoderError> for SlackError {
    fn from(err: EncoderError) -> SlackError {
        SlackError {
            kind: ErrEncoder(err),
            desc: format!("{:?}", err),
        }
    }
}

impl From<curl::ErrCode> for SlackError {
    fn from(err: curl::ErrCode) -> SlackError {
        SlackError {
            kind: ErrCurl(err),
            desc: format!("{:?}", err),
        }
    }
}

impl From<FromHexError> for SlackError {
    fn from(err: FromHexError) -> SlackError {
        SlackError {
            kind: ErrFromHex(err),
            desc: format!("{:?}", err),
        }
    }
}

impl<'a> From<(ErrorKind, &'a str)> for SlackError {
    fn from((kind, desc): (ErrorKind, &'a str)) -> SlackError {
        SlackError {
            kind: kind,
            desc: desc.to_owned(),
        }
    }
}

impl fmt::Display for SlackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "Invalid character '{}' at position {}", "ch", "idx"),
        }
    }
}

impl error::Error for SlackError {
    fn description(&self) -> &str {
        match self.kind {
            ErrUtf8(ref err) => err.description(),
            ErrFromHex(ref err) => err.description(),
            _ => &self.desc[..],
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.kind {
            ErrUtf8(ref err) => err.cause(),
            ErrFromHex(ref err) => err.cause(),
            _ => None,
        }
    }
}
