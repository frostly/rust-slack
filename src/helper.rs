use reqwest::Url;
use serde::{Serializer, Serialize};

pub fn bool_to_u8(b: bool) -> u8 {
    if b { 1u8 } else { 0u8 }
}

pub fn serialize_uri<S>(uri: &Option<Url>, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    uri.as_ref().map(|u| u.to_string()).serialize(ser)
}
