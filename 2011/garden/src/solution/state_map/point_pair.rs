pub mod point;

use crate::solution::state_map::point_pair::point::StateMapPoint;
use getset::{Getters, MutGetters};

#[derive(Clone, Getters, MutGetters)]
pub struct StateMapPointPair {
    #[getset(get = "pub", get_mut = "pub")]
    best_in: StateMapPoint,
    #[getset(get = "pub", get_mut = "pub")]
    runner_in: StateMapPoint,
}

impl StateMapPointPair {
    #[no_panic::no_panic]
    pub fn new() -> Self {
        Self {
            best_in: StateMapPoint::new(),
            runner_in: StateMapPoint::new(),
        }
    }

    #[no_panic::no_panic]
    pub fn point(&self, took_best_trail: bool) -> &StateMapPoint {
        if took_best_trail {
            &self.best_in
        } else {
            &self.runner_in
        }
    }

    #[no_panic::no_panic]
    pub fn point_mut(&mut self, took_best_trail: bool) -> &mut StateMapPoint {
        if took_best_trail {
            &mut self.best_in
        } else {
            &mut self.runner_in
        }
    }
}
