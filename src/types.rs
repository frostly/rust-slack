use std::error;
use std::str;
use rustc_serialize::hex::FromHexError;

pub use self::ErrorKind::{
    ErrSlackResp,
    ErrUtf8,
    ErrFromHex,
    ErrHexColor,
};

pub type SlackResult<T> = Result<T, SlackError>;

#[derive(Copy, Show)]
pub enum ErrorKind {
    ErrSlackResp,
    ErrUtf8(str::Utf8Error),
    ErrFromHex(FromHexError),
    ErrHexColor,
}

#[derive(Show)]
pub struct SlackError {
    pub kind: ErrorKind,
    pub desc: String,
    pub detail: Option<String>,
}

impl error::FromError<str::Utf8Error> for SlackError {

    fn from_error(err: str::Utf8Error) -> SlackError {
        SlackError {
            kind: ErrUtf8(err),
            desc:  format!("{}",err),
            detail: None,
        }
    }
}

impl error::FromError<FromHexError> for SlackError {

    fn from_error(err: FromHexError) -> SlackError {
        SlackError {
            kind: ErrFromHex(err),
            desc:  format!("{}",err),
            detail: None,
        }
    }
}

impl <'a>error::FromError<(ErrorKind, &'a str)> for SlackError {

    fn from_error((kind, desc): (ErrorKind, &'a str)) -> SlackError {
        SlackError {
            kind: kind,
            desc: desc.to_string(),
            detail: None,
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

    fn detail(&self) -> Option<String> {
        match self.kind {
            ErrUtf8(ref err) => err.detail(),
            ErrFromHex(ref err) => err.detail(),
            _ => self.detail.clone(),
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
