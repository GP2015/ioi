use crate::check;

pub fn from(fountain: u32, took_best_trail: bool) -> u32 {
    check::fountain(fountain);
    (fountain << 1) | (took_best_trail as u32)
}

pub fn to_fountain(state: u32) -> u32 {
    check::state(state);
    state >> 1
}

pub fn to_took_best_trail(state: u32) -> bool {
    check::state(state);
    state & 1 == 1
}
