use std::{error, fmt, str};
use rustc_serialize::hex::FromHexError;
use rustc_serialize::json::EncoderError;
use curl;

pub use self::ErrorKind::*;

pub type SlackResult<T> = Result<T, SlackError>;

#[derive(Copy, Debug)]
pub enum ErrorKind {
    ErrSlackResp,
    ErrUtf8(str::Utf8Error),
    ErrFromHex(FromHexError),
    ErrHexColor,
    ErrEncoder(EncoderError),
    ErrCurl(curl::ErrCode),
}

#[derive(Debug)]
pub struct SlackError {
    pub kind: ErrorKind,
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
