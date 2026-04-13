use getset::CopyGetters;

#[derive(Clone, Copy, CopyGetters)]
pub struct State {
    #[getset(get_copy = "pub")]
    fountain: u32,
    #[getset(get_copy = "pub")]
    took_best_trail: bool,
}

impl State {
    pub fn from(fountain: u32, took_best_trail: bool) -> Self {
        Self {
            fountain,
            took_best_trail,
        }
    }

    pub fn id(self, n: u32) -> usize {
        let id = ((self.fountain as usize) << 1) | usize::from(self.took_best_trail);
        assert!(id < (n as usize * 2));
        id
    }
}
