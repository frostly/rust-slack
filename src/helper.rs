pub fn bool_to_u8(b: bool) -> u8 {
    match b {
        true => 1u8,
        false => 0u8,
    }
}
