pub struct PassedMap {
    steps_data: Box<[Option<u32>]>,
    states: Vec<u32>,
}

impl PassedMap {
    pub fn new(n: u32) -> Self {
        Self {
            steps_data: vec![None; n as usize * 2].into_boxed_slice(),
            states: Vec::new(),
        }
    }

    // Returns (state, steps)
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32)> {
        self.states
            .iter()
            .map(|&state| (state, self.steps_data[state as usize].unwrap()))
    }

    pub fn contains_state(&self, state: u32) -> bool {
        self.steps_data[state as usize].is_some()
    }

    pub fn insert(&mut self, state: u32, steps: u32) {
        self.steps_data[state as usize] = Some(steps);
        self.states.push(state);
    }

    pub fn clear(&mut self) {
        for id in &self.states {
            self.steps_data[*id as usize] = None;
        }

        self.states.clear();
    }
}
