use crate::state_map::point::StateMapPoint;

pub struct StateMapPointPair {
    best_in: StateMapPoint,
    runner_in: StateMapPoint,
}

impl StateMapPointPair {
    pub fn new() -> Self {
        Self {
            best_in: StateMapPoint::new(),
            runner_in: StateMapPoint::new(),
        }
    }

    pub fn best_in(&self) -> &StateMapPoint {
        &self.best_in
    }

    pub fn best_in_mut(&mut self) -> &mut StateMapPoint {
        &mut self.best_in
    }

    pub fn runner_in(&self) -> &StateMapPoint {
        &self.runner_in
    }

    pub fn runner_in_mut(&mut self) -> &mut StateMapPoint {
        &mut self.runner_in
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
