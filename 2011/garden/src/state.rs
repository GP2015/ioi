const TOOK_BEST_TRAIL_INDEX: u32 = 0;
const FOUNTAIN_START_INDEX: u32 = 1;
const FOUNTAIN_SIZE: u32 = 18;

pub fn from(fountain: u32, took_best_trail: bool) -> u32 {
    (fountain << FOUNTAIN_START_INDEX) | ((took_best_trail as u32) << TOOK_BEST_TRAIL_INDEX)
}

pub fn to_fountain(state: u32) -> u32 {
    (state >> FOUNTAIN_START_INDEX) & ((1 << FOUNTAIN_SIZE) - 1)
}

pub fn to_took_best_trail(state: u32) -> bool {
    (state >> TOOK_BEST_TRAIL_INDEX) & 1 == 1
}
