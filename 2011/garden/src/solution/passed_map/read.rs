use crate::solution::state::State;

#[derive(Clone, Copy)]
pub struct StatesPassedMapRead {
    pub state: State,
    pub steps: u32,
}
