mod point_pair;

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
    point_pairs: Vec<StateMapPointPair>,
}

impl StateMap {
    delegate! {
        to |fountain: u32| self.point_pairs[fountain as usize]{

            fn best_in(&self) -> &StateMapPoint;

            fn best_in_mut(&mut self) -> &mut StateMapPoint;

            fn runner_in(&self) -> &StateMapPoint;

            fn runner_in_mut(&mut self) -> &mut StateMapPoint;

            pub fn point(&self, took_best_trail: bool) -> &StateMapPoint;

            fn point_mut(&mut self, took_best_trail: bool) -> &mut StateMapPoint;
        }
    }

    fn point_state(&self, state: State) -> &StateMapPoint {
        self.point_pairs[state.fountain() as usize].point(state.took_best_trail())
    }

    // #[no_panic::no_panicpoint_state_mut]
    fn point_state_mut(&mut self, state: State) -> &mut StateMapPoint {
        self.point_pairs[state.fountain() as usize].point_mut(state.took_best_trail())
    }

    pub fn from(n: u32, m: u32, p: u32, r: &RF) -> Self {
        let mut map = Self {
            point_pairs: vec![StateMapPointPair::new(); n as usize],
        };

        map.add_next_states(m, r);
        map.add_return_states();
        map.add_distances_to_p(n, p);

        map
    }

    fn add_next_states(&mut self, m: u32, r: &RF) {
        for trail in 0..m {
            for side in [false, true] {
                let current_fountain = r.get(trail, side);
                let next_fountain = r.get(trail, !side);

                if self.best_in(current_fountain).next_state().is_some() {
                    continue;
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

                break;
            }
        }
    }

    #[cfg(not(feature = "par"))]
    fn add_return_states(&mut self) {
        for pair in &mut self.point_pairs {
            if pair.best_in().next_state().is_none() {
                let state = pair
                    .runner_in()
                    .next_state()
                    .expect("all states have at least one path in, so best_in is defined");
                pair.best_in_mut().set_next_state(state);
            }
        }
    }

    #[cfg(feature = "par")]
    fn add_return_states(&mut self) {
        self.point_pairs.par_iter_mut().for_each(|pair| {
            if pair.best_in().next_state().is_none() {
                let next_state = pair
                    .runner_in()
                    .next_state()
                    .expect("all states have at least one path in, so best_in is defined");
                pair.best_in_mut().set_next_state(next_state);
            }
        });
    }

    fn add_distances_to_p(&mut self, n: u32, p: u32) {
        let mut states_passed_map = StatesPassedMap::new(n);

        for fountain in 0..n {
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
                .expect("all states have had their next states set");

            step_counter += 1;
        }
    }
}
