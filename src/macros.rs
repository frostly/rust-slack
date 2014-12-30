#![macro_escape]

macro_rules! fail {
    ($expr:expr) => (
        return Err(::std::error::FromError::from_error($expr));
    )
}
