use crate::{fountains::Fountains, state::State};

pub struct StateTransitions {
    data: Vec<[Option<State>; 2]>,
}

impl StateTransitions {
    pub fn from(fountains: &Fountains, n: u32) -> Self {
        let mut transitions = Self {
            data: vec![[None; 2]; n as usize],
        };

        for fountain in 0..n {
            for took_best_trail in [true, false] {
                let outbound = fountains.get(fountain).outbound(took_best_trail);
                let best_trail_next = fountains
                    .get(outbound.fountain)
                    .is_best_trail(outbound.trail);
                let next_state = State::from(outbound.fountain, best_trail_next);
                transitions.data[fountain as usize][took_best_trail as usize] = Some(next_state);
            }
        }

        transitions
    }

    pub fn get(&self, current_state: State) -> Option<State> {
        let fountain = current_state.fountain as usize;
        let took_best_trail = current_state.took_best_trail as usize;
        self.data[fountain][took_best_trail]
    }
}
