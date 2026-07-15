mod point;

use crate::solution::{
    pmap::PMap,
    state::State,
    state_map::point::{PHitInfo, StateMapPoint},
};

pub struct StateMap {
    pub best: Box<[StateMapPoint]>,
    pub runner: Box<[StateMapPoint]>,
    n: u32,
}

impl StateMap {
    pub fn from(n: u32, p: u32, r: &[[u32; 2]]) -> Self {
        let mut map = Self {
            best: vec![StateMapPoint::new(); n as usize].into_boxed_slice(),
            runner: vec![StateMapPoint::new(); n as usize].into_boxed_slice(),
            n,
        };

        map.add_next_states(r);
        map.add_return_states();
        map.add_distances_to_p(p);

        map
    }

    pub fn points(&self, best: bool) -> &[StateMapPoint] {
        if best { &self.best } else { &self.runner }
    }

    fn points_mut(&mut self, best: bool) -> &mut [StateMapPoint] {
        if best {
            &mut self.best
        } else {
            &mut self.runner
        }
    }

    fn point_from_state(&self, state: State) -> &StateMapPoint {
        &self.points(state.best)[state.fount as usize]
    }

    fn point_from_state_mut(&mut self, state: State) -> &mut StateMapPoint {
        &mut self.points_mut(state.best)[state.fount as usize]
    }

    fn add_next_states(&mut self, r: &[[u32; 2]]) {
        for [current_fount, next_fount] in r {
            if self.add_next_state(*current_fount, *next_fount) {
                self.add_next_state(*next_fount, *current_fount);
            }
        }
    }

    fn add_next_state(&mut self, current_fount: u32, next_fount: u32) -> bool {
        if self.best[current_fount as usize].next_state.is_some() {
            return true;
        }

        let best = self.runner[current_fount as usize].next_state.is_none();
        let next_best;

        if self.runner[next_fount as usize].next_state.is_some() {
            next_best = false;

            if self.best[next_fount as usize].next_state.is_none() {
                let state = State {
                    fount: current_fount,
                    best,
                };
                self.best[next_fount as usize].next_state = Some(state);
            }
        } else {
            next_best = true;

            let state = State {
                fount: current_fount,
                best,
            };
            self.runner[next_fount as usize].next_state = Some(state);
        }

        let state = State {
            fount: next_fount,
            best: next_best,
        };
        self.points_mut(!best)[current_fount as usize].next_state = Some(state);

        false
    }

    fn add_return_states(&mut self) {
        for fount in 0..self.n {
            if self.best[fount as usize].next_state.is_none() {
                let state = self.runner[fount as usize].next_state.unwrap();
                self.best[fount as usize].next_state = Some(state);
            }
        }
    }

    fn add_distances_to_p(&mut self, p: u32) {
        let mut states_passed_map = PMap::new(self.n);

        for fount in 0..self.n {
            for best in [true, false] {
                self.add_distance_to_p_of_state(State { fount, best }, &mut states_passed_map, p);
            }
        }
    }

    fn add_distance_to_p_of_state(
        &mut self,
        mut current_state: State,
        states_passed_map: &mut PMap,
        p: u32,
    ) {
        states_passed_map.clear();

        if self.point_from_state(current_state).found_if_can_reach_p {
            return;
        }

        let mut step_counter = 0;
        let mut check_if_p = false;

        loop {
            if check_if_p && current_state.fount == p {
                for read in states_passed_map.iter() {
                    let steps_to_p = step_counter - read.steps;
                    let p_best = current_state.best;
                    let p_hit_info = PHitInfo {
                        steps_to: steps_to_p,
                        best: p_best,
                    };
                    self.point_from_state_mut(read.state)
                        .set_p_hit_info(p_hit_info);
                }
                break;
            }

            check_if_p = true;

            if self.point_from_state(current_state).found_if_can_reach_p {
                if let Some(p_hit_info) = self.point_from_state(current_state).p_hit_info {
                    for read in states_passed_map.iter() {
                        let steps = step_counter - read.steps + p_hit_info.steps_to;
                        let read_p_hit_info = PHitInfo {
                            steps_to: steps,
                            best: p_hit_info.best,
                        };
                        self.point_from_state_mut(read.state)
                            .set_p_hit_info(read_p_hit_info);
                    }
                } else {
                    for read in states_passed_map.iter() {
                        self.point_from_state_mut(read.state).set_cannot_reach_p();
                    }
                }
                break;
            }

            if states_passed_map.contains_state(current_state) {
                for read in states_passed_map.iter() {
                    self.point_from_state_mut(read.state).set_cannot_reach_p();
                }
                break;
            }

            states_passed_map.insert(current_state, step_counter);

            current_state = self.point_from_state(current_state).next_state.unwrap();
            step_counter += 1;
        }
    }
}
