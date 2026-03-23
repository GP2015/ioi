#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.fountain, self.took_best_trail)
    }
}
