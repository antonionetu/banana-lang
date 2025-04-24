pub fn to_i32(option: Option<u32>) -> i32 {
    option.unwrap_or(0) as i32
}
