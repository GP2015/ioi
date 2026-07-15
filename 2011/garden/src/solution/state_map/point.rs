use crate::solution::state::State;

#[derive(Clone, Copy)]
pub struct PHitInfo {
    pub steps_to: u32,
    pub best: bool,
}

#[derive(Clone)]
pub struct StateMapPoint {
    pub next_state: Option<State>,
    pub found_if_can_reach_p: bool,
    pub p_hit_info: Option<PHitInfo>,
}

impl StateMapPoint {
    pub fn new() -> Self {
        Self {
            next_state: None,
            found_if_can_reach_p: false,
            p_hit_info: None,
        }
    }

    pub fn set_cannot_reach_p(&mut self) {
        self.found_if_can_reach_p = true;
    }

    pub fn set_p_hit_info(&mut self, p_hit_info: PHitInfo) {
        self.found_if_can_reach_p = true;
        self.p_hit_info = Some(p_hit_info);
    }
}
