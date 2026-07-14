pub mod point;

use crate::solution::state_map::point_pair::point::StateMapPoint;

#[derive(Clone)]
pub struct StateMapPointPair {
    pub best_in: StateMapPoint,
    pub runner_in: StateMapPoint,
}

impl StateMapPointPair {
    pub fn new() -> Self {
        Self {
            best_in: StateMapPoint::new(),
            runner_in: StateMapPoint::new(),
        }
    }

    pub fn point(&self, took_best_trail: bool) -> &StateMapPoint {
        if took_best_trail {
            &self.best_in
        } else {
            &self.runner_in
        }
    }

    pub fn point_mut(&mut self, took_best_trail: bool) -> &mut StateMapPoint {
        if took_best_trail {
            &mut self.best_in
        } else {
            &mut self.runner_in
        }
    }
}
