pub mod read;

use crate::solution::{passed_map::read::StatesPassedMapRead, state::State};

pub struct StatesPassedMap {
    steps_data: Vec<Option<u32>>,
    states: Vec<State>,
}

impl StatesPassedMap {
    // #[no_panic::no_panic]
    pub fn new(n: u32) -> Self {
        Self {
            steps_data: vec![None; n as usize * 2],
            states: Vec::new(),
        }
    }

    #[no_panic::no_panic]
    pub fn iter(&self) -> impl Iterator<Item = StatesPassedMapRead> {
        self.states.iter().map(|&state| {
            StatesPassedMapRead::from(
                state,
                self.steps_data[state.id()]
                    .expect("since this index exists in 'states', it must not be None"),
            )
        })
    }

    // #[no_panic::no_panic]
    pub fn contains_state(&self, state: State) -> bool {
        self.steps_data[state.id()].is_some()
    }

    // #[no_panic::no_panic]
    pub fn insert(&mut self, state: State, steps: u32) {
        self.steps_data[state.id()] = Some(steps);
        self.states.push(state);
    }

    // #[no_panic::no_panic]
    pub fn clear(&mut self) {
        self.states
            .iter()
            .for_each(|&state| self.steps_data[state.id()] = None);

        self.states.clear();
    }
}
