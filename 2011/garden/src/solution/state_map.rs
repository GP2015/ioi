mod point_pair;

use std::hint;

use crate::{
    RF,
    solution::{
        passed_map::StatesPassedMap,
        state::State,
        state_map::point_pair::{
            StateMapPointPair,
            point::{StateMapPoint, p_hit_info::PHitInfo},
        },
    },
};
use delegate::delegate;
#[cfg(feature = "par")]
use rayon::iter::{IntoParallelRefMutIterator as _, ParallelIterator as _};

pub struct StateMap {
    point_pairs: Box<[StateMapPointPair]>,
}

impl StateMap {
    pub fn n(&self) -> u32 {
        self.point_pairs.len() as u32
    }

    fn point_pair(&self, fountain: u32) -> &StateMapPointPair {
        self.point_pairs.get(fountain as usize).unwrap_or_else(|| {
            // Safety: fountain must be a valid index.
            unsafe { hint::unreachable_unchecked() }
        })
    }

    fn point_pair_mut(&mut self, fountain: u32) -> &mut StateMapPointPair {
        self.point_pairs
            .get_mut(fountain as usize)
            .unwrap_or_else(|| {
                // Safety: fountain must be a valid index.
                unsafe { hint::unreachable_unchecked() }
            })
    }

    delegate! {
        to |fountain: u32| self.point_pair(fountain) {
            fn best_in(&self) -> &StateMapPoint;
            fn runner_in(&self) -> &StateMapPoint;
            pub fn point(&self, took_best_trail: bool) -> &StateMapPoint;
        }

        to |fountain: u32| self.point_pair_mut(fountain) {
            fn best_in_mut(&mut self) -> &mut StateMapPoint;
            fn runner_in_mut(&mut self) -> &mut StateMapPoint;
            fn point_mut(&mut self, took_best_trail: bool) -> &mut StateMapPoint;
        }
    }

    fn point_state(&self, state: State) -> &StateMapPoint {
        self.point_pair(state.fountain())
            .point(state.took_best_trail())
    }

    fn point_state_mut(&mut self, state: State) -> &mut StateMapPoint {
        self.point_pair_mut(state.fountain())
            .point_mut(state.took_best_trail())
    }

    pub fn from(n: u32, p: u32, r: &RF) -> Self {
        let mut map = Self {
            point_pairs: vec![StateMapPointPair::new(); n as usize].into_boxed_slice(),
        };

        map.add_next_states(r);
        map.add_return_states();
        map.add_distances_to_p(p);

        map
    }

    fn add_next_states(&mut self, r: &RF) {
        for (current_fountain, next_fountain) in r.iter() {
            if self.add_next_state(current_fountain, next_fountain) {
                self.add_next_state(next_fountain, current_fountain);
            }
        }
    }

    fn add_next_state(&mut self, current_fountain: u32, next_fountain: u32) -> bool {
        if self.best_in(current_fountain).next_state().is_some() {
            return true;
        }

        let took_best_trail = self.runner_in(current_fountain).next_state().is_none();
        let next_took_best_trail;

        if self.runner_in(next_fountain).next_state().is_some() {
            next_took_best_trail = false;

            if self.best_in(next_fountain).next_state().is_none() {
                let state = State::from(current_fountain, took_best_trail);
                self.best_in_mut(next_fountain).set_next_state(state);
            }
        } else {
            next_took_best_trail = true;

            let state = State::from(current_fountain, took_best_trail);
            self.runner_in_mut(next_fountain).set_next_state(state);
        }

        let state = State::from(next_fountain, next_took_best_trail);
        self.point_mut(current_fountain, !took_best_trail)
            .set_next_state(state);

        false
    }

    #[cfg(not(feature = "par"))]
    fn add_return_states(&mut self) {
        for pair in &mut self.point_pairs {
            if pair.best_in().next_state().is_none() {
                let state = pair.runner_in().next_state().unwrap_or_else(|| {
                    // Safety: all fountains should have at least one path in, so runner_in is defined.
                    unsafe { hint::unreachable_unchecked() }
                });
                pair.best_in_mut().set_next_state(state);
            }
        }
    }

    #[cfg(feature = "par")]
    fn add_return_states(&mut self) {
        self.point_pairs.par_iter_mut().for_each(|pair| {
            if pair.best_in().next_state().is_none() {
                let next_state = pair.runner_in().next_state().unwrap_or_else(|| {
                    // Safety: all fountains should have at least one path in, so runner_in is defined.
                    unsafe { hint::unreachable_unchecked() }
                });
                pair.best_in_mut().set_next_state(next_state);
            }
        });
    }

    fn add_distances_to_p(&mut self, p: u32) {
        let mut states_passed_map = StatesPassedMap::new(self.n());

        for fountain in 0..self.n() {
            for took_best_trail in [true, false] {
                self.add_distance_to_p_of_state(
                    State::from(fountain, took_best_trail),
                    &mut states_passed_map,
                    p,
                );
            }
        }
    }

    fn add_distance_to_p_of_state(
        &mut self,
        mut current_state: State,
        states_passed_map: &mut StatesPassedMap,
        p: u32,
    ) {
        states_passed_map.clear();

        if self.point_state(current_state).found_if_can_reach_p() {
            return;
        }

        let mut step_counter = 0;
        let mut check_if_p = false;

        loop {
            if check_if_p && current_state.fountain() == p {
                for read in states_passed_map.iter() {
                    let steps_to_p = step_counter - read.steps();
                    let p_took_best_trail = current_state.took_best_trail();
                    let p_hit_info = PHitInfo::from(steps_to_p, p_took_best_trail);
                    self.point_state_mut(read.state())
                        .set_p_hit_info(p_hit_info);
                }
                break;
            }

            check_if_p = true;

            if self.point_state(current_state).found_if_can_reach_p() {
                if let Some(p_hit_info) = self.point_state(current_state).p_hit_info() {
                    for read in states_passed_map.iter() {
                        let steps = step_counter - read.steps() + p_hit_info.steps_to();
                        let read_p_hit_info = PHitInfo::from(steps, p_hit_info.took_best_trail());
                        self.point_state_mut(read.state())
                            .set_p_hit_info(read_p_hit_info);
                    }
                } else {
                    for read in states_passed_map.iter() {
                        self.point_state_mut(read.state()).set_cannot_reach_p();
                    }
                }
                break;
            }

            if states_passed_map.contains_state(current_state) {
                for read in states_passed_map.iter() {
                    self.point_state_mut(read.state()).set_cannot_reach_p();
                }
                break;
            }

            states_passed_map.insert(current_state, step_counter);

            current_state = self
                .point_state(current_state)
                .next_state()
                .unwrap_or_else(|| {
                    // Safety: all states have had their next states set already.
                    unsafe { hint::unreachable_unchecked() }
                });

            step_counter += 1;
        }
    }
}
