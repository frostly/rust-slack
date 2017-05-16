error_chain! {
    foreign_links {
        Utf8(::std::str::Utf8Error) #[doc = "utf8 error, slack responses should be valid utf8"];
        Serialize(::serde_json::error::Error) #[doc = "`serde_json::error::Error`"];
        FromHex(::hexx::FromHexError) #[doc = "`rustc_serialize::hex::FromHexError`"];
        Curl(::curl::Error) #[doc = "curl error"];
        Url(::url::ParseError) #[doc = "`Url` parsing error"];
    }
    errors {
        /// slack service error
        Slack(err: String) {
            description("slack service error")
            display("slack service error: {}", err)
        }
        /// `HexColor` parsing error
        HexColor(err: String) {
            description("hex color parsing error")
            display("hex color parsing error: {}", err)
        }
    }
}
