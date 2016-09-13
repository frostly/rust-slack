use hexx;
use curl;
use serde_json;

/// `Result` type-alias
pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    /// Errors
    #[derive(Debug)]
    pub enum Error {
        /// Slack service error
        Slack(err: String) {
            description("slack error")
            display("slack error: {}", err)
        }
        /// utf8 error, slack responses should be valid utf8
        Utf8(err: ::std::str::Utf8Error) {
            from()
            description("utf8 error")
            display("utf8 error: {}", err)
            cause(err)
        }
        /// `serde_json::error::Error`
        Serialize(err: serde_json::error::Error) {
            from()
            description("serde_json::error::Error")
            display("serde_json::error::Error: {}", err)
            cause(err)
        }
        /// `rustc_serialize::hex::FromHexError`
        FromHex(err: hexx::FromHexError) {
            from()
            description("hexx::FromHexError")
            display("hexx::FromHexError: {}", err)
            cause(err)
        }
        /// `HexColor` parsing error
        HexColor(err: String) {
            description("hex color parsing error")
            display("hex color parsing error: {}", err)
        }
        /// Curl errors
        Curl(err: curl::Error) {
            from()
            description("curl error")
            display("curl error: {}", err)
            cause(err)
        }
    }
}
