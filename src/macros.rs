#![macro_use]

macro_rules! fail {
    ($expr:expr) => (
        Err(::std::convert::From::from($expr))
    )
}
