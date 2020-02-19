use crate::error::Result;


/// A `HexColor` `String` can be one of:
///
/// 1. `String`s: `good`, `warning`, `danger`
/// 2. Any valid hex color code: e.g. `#b13d41` or `#000`.
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
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
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
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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

impl From<SlackColor> for String {
    fn from(color: SlackColor) -> String {
        color.to_string()
    }
}

impl HexColor {
    /// A checked constructor for HexColor
    ///
    /// Note that this used to be an `impl<S> TryFrom<S> for HexColor`,
    /// but due to a conflicting blanket implementation, this has been changed.
    /// https://github.com/rust-lang/rust/issues/50133
    pub fn new_checked<S: Into<String>>(s: S) -> Result<Self> {
        let s: String = s.into();
        if SLACK_COLORS.contains(&&s[..]) {
            return Ok(HexColor(s));
        }

        let num_chars = s.chars().count();
        if num_chars != 7 && num_chars != 4 {
            return Err(format_err!(
                "Must be 4 or 7 characters long (including #): \
                 found `{}`",
                s
            ));
        }
        if !s.starts_with('#') {
            return Err(format_err!("No leading #: found `{}`", s));
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
        match hex::decode(&hex[1..]) {
            Ok(_) => Ok(HexColor::new(s)),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::HexColor;

    #[test]
    fn test_hex_color_too_short() {
        let err = HexColor::new_checked("abc").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Must be 4 or 7 characters long (including #): found `abc`"
        );
    }

    #[test]
    fn test_hex_color_missing_hash() {
        let err = HexColor::new_checked("1234567").unwrap_err();
        assert_eq!(
            err.to_string(),
            "No leading #: found `1234567`"
        )
    }

    #[test]
    fn test_hex_color_invalid_hex_fmt() {
        let err = HexColor::new_checked("#abc12z").unwrap_err();
        assert!(
            err.to_string()
                .contains("Invalid character 'z' at position 5")
        );
    }

    #[test]
    fn test_hex_color_good() {
        let h: HexColor = HexColor::new_checked(SlackColor::Good).unwrap();
        assert_eq!(h.to_string(), "good");
    }

    #[test]
    fn test_hex_color_danger_str() {
        let ok = HexColor::new_checked("danger").unwrap();
        assert_eq!(ok.to_string(), "danger");
    }

    #[test]
    fn test_hex_color_3_char_hex() {
        let ok = HexColor::new_checked("#d18").unwrap();
        assert_eq!(ok.to_string(), "#d18");
    }

    #[test]
    fn test_hex_color_valid_upper_hex() {
        let ok = HexColor::new_checked("#103D18").unwrap();
        assert_eq!(ok.to_string(), "#103D18");
    }

    #[test]
    fn test_hex_color_valid_lower_hex() {
        let ok = HexColor::new_checked("#103d18").unwrap();
        assert_eq!(ok.to_string(), "#103d18");
    }
}
