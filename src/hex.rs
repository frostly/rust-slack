use crate::error::Error;
use std::{convert::TryFrom, fmt, str::FromStr};

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

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// FIXME(cosmic): Why provide this when there's already `FromStr`? Some generic bound maybe?
impl TryFrom<&str> for HexColor {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

/// Default slack colors built-in to the API
/// See: <https://api.slack.com/docs/attachments>
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

impl fmt::Display for SlackColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
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
            return Err(Error::HexColor(format!(
                "Must be 4 or 7 characters long (including #): \
                 found `{}`",
                s
            )));
        }
        if !s.starts_with('#') {
            return Err(Error::HexColor(format!("No leading #: found `{}`", s)));
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
    use std::convert::TryFrom;

    use super::*;
    use crate::HexColor;

    use insta::assert_snapshot;

    mod err {
        use super::*;

        #[test]
        fn too_short() {
            let err = HexColor::try_from("abc").unwrap_err();
            assert_snapshot!(
                err,
                @"hex color parsing error: Must be 4 or 7 characters long (including #): found `abc`"
            );
        }

        #[test]
        fn missing_hash() {
            let err = HexColor::try_from("1234567").unwrap_err();
            assert_snapshot!(
                err,
                @"hex color parsing error: No leading #: found `1234567`"
            );
        }

        #[test]
        fn invalid_hex_char() {
            let err = HexColor::try_from("#abc12z").unwrap_err();
            assert_snapshot!(err, @"Invalid character 'z' at position 5");
        }
    }

    mod ok {
        use super::*;

        fn assert_hexcolor_roundtrip(color: &str) {
            let ok: HexColor = color.parse().expect("color should be valid");
            assert_eq!(ok.to_string(), color, "Color should roundtrip");
        }

        #[test]
        fn good_variant() {
            let h: HexColor = SlackColor::Good.into();
            assert_snapshot!(h, @"good");
        }

        #[test]
        fn danger_str() {
            assert_hexcolor_roundtrip("danger");
        }

        #[test]
        fn short_hex() {
            assert_hexcolor_roundtrip("#d18");
        }

        #[test]
        fn upper_hex() {
            assert_hexcolor_roundtrip("#103D18");
        }

        #[test]
        fn lower_hex() {
            assert_hexcolor_roundtrip("#103d18");
        }
    }
}
