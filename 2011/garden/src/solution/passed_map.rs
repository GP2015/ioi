pub mod read;

use crate::solution::{passed_map::read::StatesPassedMapRead, state::State};
use std::hint;
use no_panic::no_panic;

pub struct StatesPassedMap {
    steps_data: Box<[Option<u32>]>,
    states: Vec<State>,
    n: u32,
}

impl StatesPassedMap {
    pub fn new(n: u32) -> Self {
        Self {
            steps_data: vec![None; n as usize * 2].into_boxed_slice(),
            states: Vec::new(),
            n,
        }
    }

    #[inline]
    #[no_panic]
    fn steps_data(&self, id: u32) -> Option<u32> {
        *self.steps_data.get(id as usize).unwrap_or_else(|| {
            // Safety: state.id() must be a valid index
            unsafe { hint::unreachable_unchecked() }
        })
    }

    #[inline]
    #[no_panic]
    fn steps_data_mut(&mut self, id: u32) -> &mut Option<u32> {
        self.steps_data.get_mut(id as usize).unwrap_or_else(|| {
            // Safety: state.id() must be a valid index
            unsafe { hint::unreachable_unchecked() }
        })
    }

    #[no_panic]
    pub fn iter(&self) -> impl Iterator<Item = StatesPassedMapRead> {
        self.states.iter().map(|&state| {
            StatesPassedMapRead::from(
                state,
                self.steps_data(state.id(self.n)).unwrap_or_else(|| {
                    // Safety: since its index exists in 'states', it cannot hold None
                    unsafe { hint::unreachable_unchecked() }
                }),
            )
        })
    }

    #[no_panic]
    pub fn contains_state(&self, state: State) -> bool {
        self.steps_data(state.id(self.n)).is_some()
    }

    pub fn insert(&mut self, state: State, steps: u32) {
        *self.steps_data_mut(state.id(self.n)) = Some(steps);
        self.states.push(state);
    }

    #[no_panic]
    pub fn clear(&mut self) {
        for index in 0..self.states.len() {
            #[allow(clippy::indexing_slicing)]
            let id = self.states[index].id(self.n);
            *self.steps_data_mut(id) = None;
        }

        self.states.clear();
    }
}
