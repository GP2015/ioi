#[derive(Clone, Copy)]
pub struct State {
    pub fountain: u32,
    pub took_best_trail: bool,
}

impl State {
    pub fn from(fountain: u32, took_best_trail: bool) -> Self {
        Self {
            fountain,
            took_best_trail,
        }
    }

    pub fn id(self) -> u32 {
        (self.fountain << 1) | u32::from(self.took_best_trail)
    }
}
