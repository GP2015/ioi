pub mod p_hit_info;

use crate::solution::{state::State, state_map::point_pair::point::p_hit_info::PHitInfo};
use getset::{CopyGetters, Setters};

#[derive(Clone, CopyGetters, Setters)]
pub struct StateMapPoint {
    #[getset(get_copy = "pub")]
    next_state: Option<State>,
    #[getset(get_copy = "pub")]
    found_if_can_reach_p: bool,
    #[getset(get_copy = "pub")]
    p_hit_info: Option<PHitInfo>,
}

impl StateMapPoint {
    pub fn new() -> Self {
        Self {
            next_state: None,
            found_if_can_reach_p: false,
            p_hit_info: None,
        }
    }

    pub fn set_next_state(&mut self, next_state: State) {
        self.next_state = Some(next_state);
    }

    pub fn set_cannot_reach_p(&mut self) {
        self.found_if_can_reach_p = true;
    }

    pub fn set_p_hit_info(&mut self, p_hit_info: PHitInfo) {
        self.found_if_can_reach_p = true;
        self.p_hit_info = Some(p_hit_info);
    }
}
