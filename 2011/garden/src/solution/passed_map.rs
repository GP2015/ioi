pub mod read;

use crate::solution::{passed_map::read::StatesPassedMapRead, state::State};
use std::hint;

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

    pub fn iter(&self) -> impl Iterator<Item = StatesPassedMapRead> {
        self.states.iter().map(|&state| {
            StatesPassedMapRead::from(
                state,
                self.steps_data
                    .get(state.id(self.n))
                    .unwrap_or_else(|| {
                        // Safety: state.id() must be a valid index
                        unsafe { hint::unreachable_unchecked() }
                    })
                    .unwrap_or_else(|| {
                        // Safety: since its index exists in 'states', it cannot hold None
                        unsafe { hint::unreachable_unchecked() }
                    }),
            )
        })
    }

    pub fn contains_state(&self, state: State) -> bool {
        self.steps_data
            .get(state.id(self.n))
            .unwrap_or_else(|| {
                // Safety: state.id() must be a valid index
                unsafe { hint::unreachable_unchecked() }
            })
            .is_some()
    }

    pub fn insert(&mut self, state: State, steps: u32) {
        *self
            .steps_data
            .get_mut(state.id(self.n))
            .unwrap_or_else(|| {
                // Safety: state.id() must be a valid index
                unsafe { hint::unreachable_unchecked() }
            }) = Some(steps);
        self.states.push(state);
    }

    pub fn clear(&mut self) {
        self.states.iter().for_each(|&state| {
            *self
                .steps_data
                .get_mut(state.id(self.n))
                .unwrap_or_else(|| {
                    // Safety: state.id() must be a valid index
                    unsafe { hint::unreachable_unchecked() }
                }) = None;
        });

        self.states.clear();
    }
}
