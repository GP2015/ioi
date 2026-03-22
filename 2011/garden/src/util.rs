pub fn prev_power_of_two(n: u32) -> u32 {
    let bits = 32 - n.leading_zeros();
    1 << (bits - 1)
}
