use crate::solution::state::State;
use getset::CopyGetters;

#[derive(Clone, Copy, CopyGetters)]
pub struct StatesPassedMapRead {
    #[getset(get_copy = "pub")]
    state: State,
    #[getset(get_copy = "pub")]
    steps: u32,
}

impl StatesPassedMapRead {
    #[no_panic::no_panic]
    pub fn from(state: State, steps: u32) -> Self {
        Self { state, steps }
    }
}
