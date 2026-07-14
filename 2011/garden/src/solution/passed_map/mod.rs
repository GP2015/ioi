pub mod read;

use crate::solution::{passed_map::read::StatesPassedMapRead, state::State};

pub struct StatesPassedMap {
    steps_data: Box<[Option<u32>]>,
    states: Vec<State>,
}

impl StatesPassedMap {
    pub fn new(n: u32) -> Self {
        Self {
            steps_data: vec![None; n as usize * 2].into_boxed_slice(),
            states: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = StatesPassedMapRead> {
        self.states.iter().map(|&state| StatesPassedMapRead {
            state,
            steps: self.steps_data[state.id() as usize].unwrap(),
        })
    }

    pub fn contains_state(&self, state: State) -> bool {
        self.steps_data[state.id() as usize].is_some()
    }

    pub fn insert(&mut self, state: State, steps: u32) {
        self.steps_data[state.id() as usize] = Some(steps);
        self.states.push(state);
    }

    pub fn clear(&mut self) {
        for id in self.states.iter().map(|state| state.id() as usize) {
            self.steps_data[id] = None;
        }

        self.states.clear();
    }
}
