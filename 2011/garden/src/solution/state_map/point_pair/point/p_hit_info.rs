use getset::CopyGetters;

#[derive(Clone, Copy, CopyGetters)]
pub struct PHitInfo {
    #[getset(get_copy = "pub")]
    steps_to: u32,
    #[getset(get_copy = "pub")]
    took_best_trail: bool,
}

impl PHitInfo {
    #[no_panic::no_panic]
    pub fn from(steps_to: u32, took_best_trail: bool) -> Self {
        Self {
            steps_to,
            took_best_trail,
        }
    }
}
