mod point;
mod point_pair;

use crate::{
    RF,
    passed_map::StatesPassedMap,
    state,
    state_map::{point::StateMapPoint, point_pair::StateMapPointPair},
};
use delegate::delegate;
#[cfg(feature = "par")]
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

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

    fn point_state_mut(&mut self, state: u32) -> &mut StateMapPoint {
        self.point_pairs[state::to_fountain(state) as usize]
            .point_mut(state::to_took_best_trail(state))
    }

    pub fn from(n: u32, m: u32, p: u32, r: RF) -> Self {
        let mut map = Self {
            point_pairs: (0..n).map(|_| StateMapPointPair::new()).collect(),
        };

        map.add_next_states(m, r);
        map.add_return_states();
        map.add_distances_to_p(n, p);

        map
    }

    fn add_next_states(&mut self, m: u32, r: RF) {
        for trail in 0..m {
            for side in 0..2 {
                let current_fountain = r.get(trail, side);
                let next_fountain = r.get(trail, 1 - side);

                if self.best_in(current_fountain).next_state().is_some() {
                    continue;
                }

                let is_best_trail = self.runner_in(current_fountain).next_state().is_none();
                let is_next_best_trail;

                if self.runner_in(next_fountain).next_state().is_some() {
                    is_next_best_trail = false;

                    if self.best_in(next_fountain).next_state().is_none() {
                        self.best_in_mut(next_fountain)
                            .set_next_state(current_fountain, is_best_trail);
                    }
                } else {
                    is_next_best_trail = true;

                    self.runner_in_mut(next_fountain)
                        .set_next_state(current_fountain, is_best_trail);
                }

                self.point_mut(current_fountain, !is_best_trail)
                    .set_next_state(next_fountain, is_next_best_trail);

                break;
            }
        }
    }

    fn add_return_states(&mut self) {
        #[cfg(not(feature = "par"))]
        for pair in &mut self.point_pairs {
            if pair.best_in().next_state().is_none() {
                let (next_fountain, next_took_best_trail) = pair.runner_in().next_state().unwrap();
                pair.best_in_mut()
                    .set_next_state(next_fountain, next_took_best_trail);
            }
        }

        #[cfg(feature = "par")]
        self.point_pairs.par_iter_mut().for_each(|pair| {
            if pair.best_in().next_state().is_none() {
                let (next_fountain, next_took_best_trail) = pair.runner_in().next_state().unwrap();
                pair.best_in_mut()
                    .set_next_state(next_fountain, next_took_best_trail);
            }
        })
    }

    fn add_distances_to_p(&mut self, n: u32, p: u32) {
        let mut states_passed_map = StatesPassedMap::new();

        for fountain in 0..n {
            for took_best_trail in [true, false] {
                self.add_distance_to_p_of_state(
                    fountain,
                    took_best_trail,
                    &mut states_passed_map,
                    p,
                );
            }
        }
    }

    fn add_distance_to_p_of_state(
        &mut self,
        fountain: u32,
        took_best_trail: bool,
        states_passed_map: &mut StatesPassedMap,
        p: u32,
    ) {
        states_passed_map.clear();

        if self.point(fountain, took_best_trail).found_if_can_reach_p() {
            return;
        }

        let mut current_fountain = fountain;
        let mut current_took_best_trail = took_best_trail;

        let mut step_counter = 0;
        let mut check_for_p = false;

        loop {
            if check_for_p && current_fountain == p {
                for (state, step_count) in states_passed_map.iter() {
                    let steps = step_counter - step_count;
                    let p_took_best_trail = current_took_best_trail;
                    self.point_state_mut(state)
                        .set_p_hit_info(steps, p_took_best_trail);
                }
                break;
            }

            check_for_p = true;

            if self
                .point(current_fountain, current_took_best_trail)
                .found_if_can_reach_p()
            {
                if let Some((steps_to_p, p_took_best_trail)) = self
                    .point(current_fountain, current_took_best_trail)
                    .p_hit_info()
                {
                    for (state, steps) in states_passed_map.iter() {
                        let steps = step_counter - steps + steps_to_p;
                        self.point_state_mut(state)
                            .set_p_hit_info(steps, p_took_best_trail);
                    }
                } else {
                    for (state, _) in states_passed_map.iter() {
                        self.point_state_mut(state).set_cannot_reach_p();
                    }
                }
                break;
            }

            let current_state = state::from(current_fountain, current_took_best_trail);

            if states_passed_map.contains_state(current_state) {
                for (state, _) in states_passed_map.iter() {
                    self.point_state_mut(state).set_cannot_reach_p();
                }
                break;
            }

            states_passed_map.insert(current_state, step_counter);

            (current_fountain, current_took_best_trail) = self
                .point(current_fountain, current_took_best_trail)
                .next_state()
                .expect("the point must have been given a next_state already");

            step_counter += 1;
            assert!(step_counter <= 300_000);
        }
    }
}
