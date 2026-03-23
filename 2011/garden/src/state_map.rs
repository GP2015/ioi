mod point;
mod point_pair;

use ahash::AHashMap;

use crate::{
    RF,
    state::State,
    state_map::{point::StateMapPoint, point_pair::StateMapPointPair},
};

pub struct StateMap {
    point_pairs: Vec<StateMapPointPair>,
}

impl StateMap {
    pub fn new(n: u32) -> Self {
        Self {
            point_pairs: vec![StateMapPointPair::new(); n as usize],
        }
    }

    fn best_in(&self, fountain: u32) -> &StateMapPoint {
        &self.point_pairs[fountain as usize].best_in
    }

    fn best_in_mut(&mut self, fountain: u32) -> &mut StateMapPoint {
        &mut self.point_pairs[fountain as usize].best_in
    }

    fn runner_in(&self, fountain: u32) -> &StateMapPoint {
        &self.point_pairs[fountain as usize].runner_in
    }

    fn runner_in_mut(&mut self, fountain: u32) -> &mut StateMapPoint {
        &mut self.point_pairs[fountain as usize].runner_in
    }

    pub fn point(&self, fountain: u32, took_best_trail: bool) -> &StateMapPoint {
        self.point_pairs[fountain as usize].point(took_best_trail)
    }

    fn point_mut(&mut self, fountain: u32, took_best_trail: bool) -> &mut StateMapPoint {
        self.point_pairs[fountain as usize].point_mut(took_best_trail)
    }

    fn point_state(&self, state: State) -> &StateMapPoint {
        self.point_pairs[state.fountain as usize].point(state.took_best_trail)
    }

    fn point_state_mut(&mut self, state: State) -> &mut StateMapPoint {
        self.point_pairs[state.fountain as usize].point_mut(state.took_best_trail)
    }

    pub fn add_next_states(&mut self, m: u32, r: RF) {
        for trail in 0..m {
            for side in 0..2 {
                let current_fountain = r.get(trail, side);
                let next_fountain = r.get(trail, 1 - side);

                if self.best_in(current_fountain).next_state.is_some() {
                    continue;
                }

                let is_best_trail = self.runner_in(current_fountain).next_state.is_none();
                let is_next_best_trail;

                if self.runner_in(next_fountain).next_state.is_some() {
                    is_next_best_trail = false;

                    if self.best_in(next_fountain).next_state.is_none() {
                        let next_state = State::from(current_fountain, is_best_trail);
                        self.best_in_mut(next_fountain).set_next_state(next_state);
                    }
                } else {
                    is_next_best_trail = true;

                    let next_state = State::from(current_fountain, is_best_trail);
                    self.runner_in_mut(next_fountain).set_next_state(next_state);
                }

                let next_state = State::from(next_fountain, is_next_best_trail);
                self.point_mut(current_fountain, !is_best_trail)
                    .set_next_state(next_state);

                break;
            }
        }

        for pair in &mut self.point_pairs {
            if pair.best_in.next_state.is_none() {
                let next_state = pair.runner_in.next_state.unwrap();
                pair.best_in.set_next_state(next_state);
            }
        }
    }

    pub fn add_distances_to_p(&mut self, n: u32, p: u32) {
        for fountain in 0..n {
            for took_best_trail in [true, false] {
                if self.point(fountain, took_best_trail).can_reach_p.is_some() {
                    continue;
                }

                let mut states_passed = AHashMap::<State, usize>::new();
                let mut step_counter = 0;
                let mut current_state = State::from(fountain, took_best_trail);
                let mut check_for_p = false;

                loop {
                    if check_for_p && current_state.fountain == p {
                        for (state, step_count) in states_passed {
                            let steps = step_counter - step_count;
                            let best_trail_to_p = current_state.took_best_trail;
                            self.point_state_mut(state)
                                .set_p_entry(steps, best_trail_to_p);
                        }
                        break;
                    }

                    check_for_p = true;

                    if let Some(can_reach_p) = self.point_state(current_state).can_reach_p {
                        if can_reach_p {
                            for (state, steps) in states_passed {
                                let point: &StateMapPoint = self.point_state(current_state);
                                let steps = step_counter - steps + point.steps_to_p.unwrap();
                                let best_trail_to_p = point.best_trail_to_p;
                                self.point_state_mut(state)
                                    .set_p_entry(steps, best_trail_to_p);
                            }
                        } else {
                            for (state, _) in states_passed {
                                self.point_state_mut(state).set_cannot_reach_p();
                            }
                        }
                        break;
                    }

                    if states_passed.contains_key(&current_state) {
                        for (state, _) in states_passed {
                            self.point_state_mut(state).set_cannot_reach_p();
                        }
                        break;
                    }

                    states_passed.insert(current_state, step_counter);
                    current_state = self.point_state(current_state).next_state.unwrap();
                    step_counter += 1;
                }
            }
        }
    }
}
