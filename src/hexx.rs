use crate::error::{Error, ErrorKind};
use std::{convert::TryFrom, str::FromStr};

use hex::FromHex;
use serde::Serialize;

/// A `HexColor` `String` can be one of:
///
/// 1. `String`s: `good`, `warning`, `danger`
/// 2. Any valid hex color code: e.g. `#b13d41` or `#000`.
///
/// hex color codes will be checked to ensure a valid hex number is provided
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct HexColor(String);
impl HexColor {
    fn new<S: Into<String>>(s: S) -> HexColor {
        HexColor(s.into())
    }
}

impl Default for HexColor {
    fn default() -> HexColor {
        HexColor::new("#000")
    }
}

impl ::std::fmt::Display for HexColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for HexColor {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

/// Default slack colors built-in to the API
/// See: https://api.slack.com/docs/attachments
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SlackColor {
    /// green
    Good,
    /// orange
    Warning,
    /// red
    Danger,
}

const SLACK_COLORS: [&str; 3] = ["good", "warning", "danger"];

impl ::std::fmt::Display for SlackColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for SlackColor {
    fn as_ref(&self) -> &str {
        match *self {
            SlackColor::Good => "good",
            SlackColor::Warning => "warning",
            SlackColor::Danger => "danger",
        }
    }
}

impl From<SlackColor> for HexColor {
    fn from(color: SlackColor) -> HexColor {
        HexColor::new(color.to_string())
    }
}

impl FromStr for HexColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.into();
        if SLACK_COLORS.contains(&&s[..]) {
            return Ok(HexColor(s));
        }

        let num_chars = s.chars().count();
        if num_chars != 7 && num_chars != 4 {
            return Err(ErrorKind::HexColor(format!(
                "Must be 4 or 7 characters long (including #): \
                 found `{}`",
                s
            ))
            .into());
        }
        if !s.starts_with('#') {
            return Err(ErrorKind::HexColor(format!("No leading #: found `{}`", s)).into());
        }

        // #d18 -> #dd1188
        let hex = if num_chars == 4 {
            s.chars().skip(1).fold(String::from("#"), |mut s, c| {
                s.push(c);
                s.push(c);
                s
            })
        } else {
            s.clone()
        };

        // see if the remaining part of the string is actually hex
        match Vec::from_hex(&hex[1..]) {
            Ok(_) => Ok(HexColor::new(s)),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::HexColor;
    use std::convert::TryFrom;

    #[test]
    fn test_hex_color_too_short() {
        let err = HexColor::from_str("abc").unwrap_err();
        assert_eq!(
            err.to_string(),
            "hex color parsing error: Must be 4 or 7 characters long (including #): found \
             `abc`"
        );
    }

    #[test]
    fn test_hex_color_missing_hash() {
        let err = HexColor::from_str("1234567").unwrap_err();
        assert_eq!(
            err.to_string(),
            "hex color parsing error: No leading #: found `1234567`"
        );
    }

    #[test]
    fn test_hex_color_invalid_hex_fmt() {
        let err = HexColor::from_str("#abc12z").unwrap_err();
        assert!(err
            .to_string()
            .contains("Invalid character 'z' at position 5"));
    }

    #[test]
    fn test_hex_color_good() {
        let h: HexColor = HexColor::from_str(&SlackColor::Good.to_string()).unwrap();
        assert_eq!(h.to_string(), "good");
    }

    #[test]
    fn test_hex_color_danger_str() {
        let ok = HexColor::from_str("danger").unwrap();
        assert_eq!(ok.to_string(), "danger");
    }

    #[test]
    fn test_hex_color_3_char_hex() {
        let ok = HexColor::from_str("#d18").unwrap();
        assert_eq!(ok.to_string(), "#d18");
    }

    #[test]
    fn test_hex_color_valid_upper_hex() {
        let ok = HexColor::from_str("#103D18").unwrap();
        assert_eq!(ok.to_string(), "#103D18");
    }

    #[test]
    fn test_hex_color_valid_lower_hex() {
        let ok = HexColor::from_str("#103d18").unwrap();
        assert_eq!(ok.to_string(), "#103d18");
    }
}
