use crate::solution::state::State;

#[derive(Clone, Copy)]
pub struct Read {
    pub state: State,
    pub steps: u32,
}

pub struct PMap {
    steps_data: Box<[Option<u32>]>,
    states: Vec<State>,
}

impl PMap {
    pub fn new(n: u32) -> Self {
        Self {
            steps_data: vec![None; n as usize * 2].into_boxed_slice(),
            states: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Read> {
        self.states.iter().map(|&state| Read {
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
