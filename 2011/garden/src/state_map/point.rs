use crate::state::State;

#[derive(Clone)]
pub struct StateMapPoint {
    pub next_state: Option<State>,
    pub can_reach_p: Option<bool>,
    pub steps_to_p: Option<usize>,
    pub best_trail_to_p: bool,
}

impl StateMapPoint {
    pub fn new() -> Self {
        Self {
            next_state: None,
            can_reach_p: None,
            steps_to_p: None,
            best_trail_to_p: false,
        }
    }

    pub fn set_next_state(&mut self, next_state: State) {
        self.next_state = Some(next_state);
    }

    pub fn set_cannot_reach_p(&mut self) {
        self.can_reach_p = Some(false);
    }

    pub fn set_p_entry(&mut self, steps_to_p: usize, best_trail_to_p: bool) {
        self.can_reach_p = Some(true);
        self.steps_to_p = Some(steps_to_p);
        self.best_trail_to_p = best_trail_to_p;
    }
}
