use std::fmt;
use types::{SlackResult, ErrHexColor};
use rustc_serialize::hex::{FromHex};
use rustc_serialize::json::{ToJson, Json};
use rustc_serialize::{Encodable, Encoder};

/// The HexColor string can be one of:
///
/// 1. "good", "warning", "danger"
/// 2. The built-in enums: SlackColor::Good, etc.
/// 3. Any valid hex color code: "#b13d41"
/// hex color codes will be checked to ensure a valid hex number is provided
pub struct HexColor(String);

/// Default slack colors built-in to the API
/// See: https://api.slack.com/docs/attachments
#[allow(dead_code)]
#[derive(Copy)]
pub enum SlackColor {
    Good,
    Warning,
    Danger,
}

// can't seem to convert enum to slice despite trait being implemented
// need this to support passing in the string directly
const SLACK_COLORS : [&'static str; 3] = [
    // SlackColor::Good.as_slice(),
    "good",
    "warning",
    "danger",
];


impl ToString for SlackColor {
    fn to_string(&self) -> String {
        self.as_slice().to_string()
    }
}

impl Str for SlackColor {
    fn as_slice(&self) -> &str {
        match *self {
            SlackColor::Good => "good",
            SlackColor::Warning => "warning",
            SlackColor::Danger => "danger"
        }
    }
}

impl fmt::Show for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let HexColor(ref text) = *self;
        write!(f , "{}" , text)
    }
}

pub trait HexColorT {
    type Sized? T;
    fn new(t: &Self::T) -> Self;
}

impl HexColorT for SlackResult<HexColor> {
    type T = str;
    fn new(s: &str) -> SlackResult<HexColor> {
        Ok(try!(s.into_hex_color()))
    }
}

impl HexColorT for HexColor {
    type T = SlackColor;

    fn new(color: &SlackColor) -> HexColor {
        HexColor(color.to_string())
    }
}

impl ToJson for HexColor {
    fn to_json(&self) -> Json {
        Json::String(format!("{}", &self))
    }
}

impl <S: Encoder<E>, E> Encodable<S, E> for HexColor {
  fn encode(&self, encoder: &mut S) -> Result<(), E> {
      encoder.emit_str(format!("{}", &self).as_slice())
  }
}


trait IntoHexColor {
    fn into_hex_color(self) -> SlackResult<HexColor>;
}

impl<'a> IntoHexColor for &'a str {
    fn into_hex_color(self) -> SlackResult<HexColor> {
        if SLACK_COLORS.contains(&self) {
            return Ok(HexColor(self.to_string()));
        }
        if self.chars().count() != 7u {
            return fail!((ErrHexColor, "Must be 7 characters long (including #)"));
        }
        if self.char_at(0) != '#' {
            return fail!((ErrHexColor, "No leading #"));
        }
        match self.slice_from(1).from_hex() {
            Ok(_) => Ok(HexColor(self.to_string())),
            Err(e) => Err(::std::error::FromError::from_error(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use hex::{HexColor, HexColorT, SlackColor};
    use types::{SlackResult};
    use std::error::Error;

    #[test]
    fn test_hex_color_too_short() {
        let h1: SlackResult<HexColor> =  HexColorT::new("abc");
        let h = h1.unwrap_err();
        assert_eq!(h.desc, "Must be 7 characters long (including #)".to_string());
    }

    #[test]
    fn test_hex_color_missing_hash() {
        let h1: SlackResult<HexColor> = HexColorT::new("1234567");
        let h = h1.unwrap_err();
        assert_eq!(h.desc, "No leading #".to_string());
    }

    #[test]
    fn test_hex_color_invalid_hex_fmt() {
        let h1: SlackResult<HexColor> = HexColorT::new("#abc12z");
        let h = h1.unwrap_err();
        assert_eq!(h.desc, "Invalid character 'z' at position 5".to_string());
    }

    #[test]
    fn test_hex_color_error_impl() {
        let h1: SlackResult<HexColor> = HexColorT::new("#abc12z");
        let h = h1.unwrap_err();
        assert_eq!(h.description(), "invalid character".to_string());
    }

    #[test]
    fn test_hex_color_good() {
        let h: HexColor = HexColorT::new(&SlackColor::Good);
        assert_eq!(format!("{}",h), "good".to_string());
    }

    #[test]
    fn test_hex_color_danger_str() {
        let h1: SlackResult<HexColor> = HexColorT::new("danger");
        let h = h1.unwrap();
        assert_eq!(format!("{}", h), "danger".to_string());
    }

    #[test]
    fn test_hex_color_bad_str() {
        let h1: SlackResult<HexColor> = HexColorT::new("bad");
        let h = h1.unwrap_err();
        assert_eq!(h.desc, "Must be 7 characters long (including #)".to_string());
    }
}
