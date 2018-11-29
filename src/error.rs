pub use failure::{Error, Fail, Context, Backtrace, ResultExt};

/*
// This would be a more expressive pattern by following
// https://rust-lang-nursery.github.io/failure/error-errorkind.html
// however it was hard to get to work inside TryFrom, TryInto traits
// so have left the typed error equivalents that was here before commented out
// but ready to be swapped in if they are useful.

#[derive(Debug)]
pub struct SlackError {
    inner: Context<ErrorKind>,
}

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "hex color parsing error: {}", _0)]
    HexColor(String),

    #[fail(display = "slack service error: {}", _0)]
    Slack(String),
}
*/

/// Error handling convenience type
pub type Result<T> = std::result::Result<T, Error>;
