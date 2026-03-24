use delegate::delegate;

const NEXT_TOOK_BEST_TRAIL_INDEX: u32 = 0;
const NEXT_FOUNTAIN_START_INDEX: u32 = 1;
const NEXT_FOUNTAIN_SIZE: u32 = 18;
const HAS_NEXT_STATE_INDEX: u32 = 19;
const HAS_CAN_REACH_P_INDEX: u32 = 20;
const CAN_REACH_P_INDEX: u32 = 21;
const HAS_P_HIT_INFO_INDEX: u32 = 22;
const P_TOOK_BEST_TRAIL_INDEX: u32 = 23;

#[derive(Clone)]
pub struct StateMapPoint {
    multi_data: u32,
    steps_to_p: u32,
}

impl StateMapPoint {
    pub fn new() -> Self {
        Self {
            multi_data: 0,
            steps_to_p: 0,
        }
    }

    fn bit(&self, bit: u32) -> bool {
        (self.multi_data >> bit) & 1 == 1
    }

    fn set_bit(&mut self, bit: u32, state: bool) {
        if state {
            self.multi_data |= 1 << bit;
        } else {
            self.multi_data &= !(1 << bit);
        }
    }

    pub fn next_fountain(&self) -> u32 {
        (self.multi_data >> NEXT_FOUNTAIN_START_INDEX) & ((1 << NEXT_FOUNTAIN_SIZE) - 1)
    }

    fn set_next_fountain(&mut self, next_fountain: u32) {
        let mask = !(((1 << NEXT_FOUNTAIN_SIZE) - 1) << NEXT_FOUNTAIN_START_INDEX);
        let multi_data_masked = self.multi_data & mask;
        self.multi_data = multi_data_masked | (next_fountain << NEXT_FOUNTAIN_START_INDEX);
    }

    delegate! {
        to self{
            #[call(bit)] pub fn has_next_state(&self, [ HAS_NEXT_STATE_INDEX ]) -> bool;
            #[call(bit)] pub fn next_took_best_trail(&self, [ NEXT_TOOK_BEST_TRAIL_INDEX ]) -> bool;
            #[call(bit)] pub fn has_can_reach_p(&self, [ HAS_CAN_REACH_P_INDEX ]) -> bool;
            #[call(bit)] pub fn can_reach_p(&self, [ CAN_REACH_P_INDEX ]) -> bool;
            #[call(bit)] pub fn has_p_hit_info(&self, [ HAS_P_HIT_INFO_INDEX ]) -> bool;
            #[call(bit)] pub fn p_took_best_trail(&self, [ P_TOOK_BEST_TRAIL_INDEX ]) -> bool;
        }
    }

    pub fn steps_to_p(&self) -> u32 {
        self.steps_to_p
    }

    pub fn set_next_state(&mut self, next_fountain: u32, next_took_best_trail: bool) {
        self.set_bit(HAS_NEXT_STATE_INDEX, true);
        self.set_bit(NEXT_TOOK_BEST_TRAIL_INDEX, next_took_best_trail);
        self.set_next_fountain(next_fountain);
    }

    pub fn set_cannot_reach_p(&mut self) {
        self.set_bit(HAS_CAN_REACH_P_INDEX, true);
        self.set_bit(CAN_REACH_P_INDEX, false);
    }

    pub fn set_p_hit_info(&mut self, steps_to_p: u32, p_took_best_trail: bool) {
        self.set_bit(HAS_CAN_REACH_P_INDEX, true);
        self.set_bit(CAN_REACH_P_INDEX, true);
        self.set_bit(HAS_P_HIT_INFO_INDEX, true);
        self.set_bit(P_TOOK_BEST_TRAIL_INDEX, p_took_best_trail);
        self.steps_to_p = steps_to_p;
    }
}
