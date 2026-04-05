use crate::check;

const STEPS_DATA_SIZE: usize = 300_000;

pub struct StatesPassedMap {
    steps_data: Box<[u32; STEPS_DATA_SIZE]>,
    states: Vec<u32>,
}

impl StatesPassedMap {
    pub fn new() -> Self {
        Self {
            steps_data: Box::new([0; STEPS_DATA_SIZE]),
            states: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, u32)> {
        self.states
            .iter()
            .map(|&state| (state, self.steps_data[state as usize] >> 1))
    }

    pub fn contains_state(&self, state: u32) -> bool {
        check::state(state);
        self.steps_data[state as usize] & 1 == 1
    }

    pub fn insert(&mut self, state: u32, steps: u32) {
        check::state(state);
        check::steps(steps);
        self.steps_data[state as usize] = (steps << 1) | 1;
        self.states.push(state);
    }

    pub fn clear(&mut self) {
        self.states
            .iter()
            .for_each(|&state| self.steps_data[state as usize] = 0);

        self.states.clear();
    }
}
