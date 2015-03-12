use std::{error, fmt, str};
use rustc_serialize::hex::FromHexError;
use rustc_serialize::json::EncoderError;
use curl;

pub use self::ErrorKind::*;

/// Result alias to save typing
pub type SlackResult<T> = Result<T, SlackError>;

/// Different kinds of errors handled
#[derive(Copy, Debug)]
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

impl error::FromError<str::Utf8Error> for SlackError {
    fn from_error(err: str::Utf8Error) -> SlackError {
        SlackError {
            kind: ErrUtf8(err),
            desc: format!("{:?}", err),
        }
    }
}

impl error::FromError<EncoderError> for SlackError {
    fn from_error(err: EncoderError) -> SlackError {
        SlackError {
            kind: ErrEncoder(err),
            desc: format!("{:?}", err),
        }
    }
}

impl error::FromError<curl::ErrCode> for SlackError {
    fn from_error(err: curl::ErrCode) -> SlackError {
        SlackError {
            kind: ErrCurl(err),
            desc: format!("{:?}", err),
        }
    }
}

impl error::FromError<FromHexError> for SlackError {
    fn from_error(err: FromHexError) -> SlackError {
        SlackError {
            kind: ErrFromHex(err),
            desc: format!("{:?}", err),
        }
    }
}

impl <'a>error::FromError<(ErrorKind, &'a str)> for SlackError {
    fn from_error((kind, desc): (ErrorKind, &'a str)) -> SlackError {
        SlackError {
            kind: kind,
            desc: desc.to_string(),
        }
    }
}

impl fmt::Display for SlackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ =>
                write!(f, "Invalid character '{}' at position {}", "ch", "idx"),
        }
    }
}

impl error::Error for SlackError {
    fn description(&self) -> &str {
        match self.kind {
            ErrUtf8(ref err) => err.description(),
            ErrFromHex(ref err) => err.description(),
            _ => self.desc.as_slice(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.kind {
            ErrUtf8(ref err) => Some(err as &error::Error),
            ErrFromHex(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }
}
