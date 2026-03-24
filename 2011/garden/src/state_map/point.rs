#[derive(Clone)]
pub struct StateMapPoint {
    next_state: Option<(u32, bool)>,
    found_if_can_reach_p: bool,
    p_hit_info: Option<(u32, bool)>,
}

impl StateMapPoint {
    pub fn new() -> Self {
        Self {
            next_state: None,
            found_if_can_reach_p: false,
            p_hit_info: None,
        }
    }

    pub fn next_state(&self) -> Option<(u32, bool)> {
        self.next_state
    }

    pub fn found_if_can_reach_p(&self) -> bool {
        self.found_if_can_reach_p
    }

    pub fn p_hit_info(&self) -> Option<(u32, bool)> {
        self.p_hit_info
    }

    pub fn set_next_state(&mut self, next_fountain: u32, next_took_best_trail: bool) {
        assert!(next_fountain < 150_000);
        self.next_state = Some((next_fountain, next_took_best_trail));
    }

    pub fn set_cannot_reach_p(&mut self) {
        self.found_if_can_reach_p = true;
    }

    pub fn set_p_hit_info(&mut self, steps_to_p: u32, p_took_best_trail: bool) {
        assert!(steps_to_p <= 300_000);
        self.found_if_can_reach_p = true;
        self.p_hit_info = Some((steps_to_p, p_took_best_trail));
    }
}
