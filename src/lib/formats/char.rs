pub fn to_i32(c: char) -> i32 {
    c.to_digit(10)
        .map(|d| d as i32)
        .unwrap_or_else(|| panic!("Invalid character for conversion: '{}'", c))
}
