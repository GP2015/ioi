pub mod read;

use crate::solution::{passed_map::read::StatesPassedMapRead, state::State};

pub struct StatesPassedMap {
    steps_data: Box<[Option<u32>]>,
    states: Vec<State>,
}

impl StatesPassedMap {
    fn n(&self) -> u32 {
        self.steps_data.len() as u32
    }

    pub fn new(n: u32) -> Self {
        Self {
            steps_data: vec![None; n as usize * 2].into_boxed_slice(),
            states: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = StatesPassedMapRead> {
        self.states.iter().map(|&state| {
            StatesPassedMapRead::from(
                state,
                self.steps_data[state.id(self.n())]
                    .expect("since its index exists in 'states', it cannot hold None"),
            )
        })
    }

    pub fn contains_state(&self, state: State) -> bool {
        self.steps_data[state.id(self.n())].is_some()
    }

    pub fn insert(&mut self, state: State, steps: u32) {
        let n = self.n();
        self.steps_data[state.id(n)] = Some(steps);
        self.states.push(state);
    }

    pub fn clear(&mut self) {
        let n = self.n();
        self.states
            .iter()
            .for_each(|&state| self.steps_data[state.id(n)] = None);

        self.states.clear();
    }
}
