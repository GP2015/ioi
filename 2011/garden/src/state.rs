pub fn from(fountain: u32, took_best_trail: bool) -> u32 {
    (fountain << 1) | (took_best_trail as u32)
}

pub fn to_fountain(state: u32) -> u32 {
    state >> 1
}

pub fn to_took_best_trail(state: u32) -> bool {
    state & 1 == 1
}
