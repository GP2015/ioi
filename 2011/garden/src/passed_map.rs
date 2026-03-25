const STEPS_DATA_SIZE: usize = 300_000;

pub struct StatesPassedMap {
    steps_data: Vec<u32>,
    states: Vec<u32>,
}

impl StatesPassedMap {
    pub fn new() -> Self {
        Self {
            steps_data: vec![0; STEPS_DATA_SIZE],
            states: Vec::with_capacity(299_000),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, u32)> {
        self.states
            .iter()
            .map(|&state| (state, self.steps_data[state as usize] >> 1))
    }

    pub fn contains_state(&self, state: u32) -> bool {
        self.steps_data[state as usize] & 1 == 1
    }

    pub fn insert(&mut self, state: u32, steps: u32) {
        self.steps_data[state as usize] = (steps << 1) | 1;
        self.states.push(state);
        assert!(self.states.len() < 300_000);
    }

    pub fn clear(&mut self) {
        for &state in &self.states {
            self.steps_data[state as usize] = 0;
        }

        self.states.clear();
    }
}
