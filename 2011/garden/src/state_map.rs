mod point;
mod point_pair;

use crate::{
    RF,
    passed_map::StatesPassedMap,
    state,
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

    fn point_state_mut(&mut self, state: u32) -> &mut StateMapPoint {
        self.point_pairs[state::to_fountain(state) as usize]
            .point_mut(state::to_took_best_trail(state))
    }

    pub fn add_next_states(&mut self, m: u32, r: RF) {
        for trail in 0..m {
            for side in 0..2 {
                let current_fountain = r.get(trail, side);
                let next_fountain = r.get(trail, 1 - side);

                if self.best_in(current_fountain).has_next_state() {
                    continue;
                }

                let is_best_trail = !self.runner_in(current_fountain).has_next_state();
                let is_next_best_trail;

                if self.runner_in(next_fountain).has_next_state() {
                    is_next_best_trail = false;

                    if !self.best_in(next_fountain).has_next_state() {
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

        for pair in &mut self.point_pairs {
            if !pair.best_in.has_next_state() {
                let next_fountain = pair.runner_in.next_fountain();
                let next_took_best_trail = pair.runner_in.next_took_best_trail();
                pair.best_in
                    .set_next_state(next_fountain, next_took_best_trail);
            }
        }

        // for fountain in 0..self.point_pairs.len() {
        //     for took_best_trail in [true, false] {
        //         let point = self.point(fountain as u32, took_best_trail);
        //         println!(
        //             "({}, {}), ({}, {})",
        //             fountain,
        //             took_best_trail,
        //             point.next_fountain(),
        //             point.next_took_best_trail()
        //         );
        //     }
        // }
    }

    pub fn add_distances_to_p(&mut self, n: u32, p: u32) {
        let mut states_passed_map = StatesPassedMap::new();

        for fountain in 0..n {
            for took_best_trail in [true, false] {
                states_passed_map.clear();

                if self.point(fountain, took_best_trail).has_can_reach_p() {
                    continue;
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
                        .has_can_reach_p()
                    {
                        if self
                            .point(current_fountain, current_took_best_trail)
                            .can_reach_p()
                        {
                            for (state, steps) in states_passed_map.iter() {
                                let point: &StateMapPoint =
                                    self.point(current_fountain, current_took_best_trail);
                                let steps = step_counter - steps + point.steps_to_p();
                                let p_took_best_trail = point.p_took_best_trail();
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

                    let next_fountain = self
                        .point(current_fountain, current_took_best_trail)
                        .next_fountain();
                    let next_took_best_trail = self
                        .point(current_fountain, current_took_best_trail)
                        .next_took_best_trail();

                    current_fountain = next_fountain;
                    current_took_best_trail = next_took_best_trail;

                    step_counter += 1;
                }
            }
        }

        // for fountain in 0..self.point_pairs.len() {
        //     for took_best_trail in [true, false] {
        //         let point = self.point(fountain as u32, took_best_trail);
        //         println!(
        //             "({}, {}), {}, {}, {}, {}, {}",
        //             fountain,
        //             took_best_trail,
        //             point.has_can_reach_p(),
        //             point.can_reach_p(),
        //             point.has_p_hit_info(),
        //             point.steps_to_p(),
        //             point.p_took_best_trail(),
        //         );
        //     }
        // }
    }
}
