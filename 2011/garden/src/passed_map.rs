const STEPS_DATA_SIZE: usize = 300_000;

pub struct StatesPassedMap {
    steps_data: Vec<Option<u32>>,
    states: Vec<u32>,
}

impl StatesPassedMap {
    pub fn new() -> Self {
        Self {
            steps_data: vec![None; STEPS_DATA_SIZE],
            states: Vec::new(),
        }
    }

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
        for &state in &self.states {
            self.steps_data[state as usize] = None;
        }

        self.states.clear();
    }
}
