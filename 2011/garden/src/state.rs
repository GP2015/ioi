#[derive(Copy, Clone, Eq, PartialEq)]
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

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mask = (1 << 18) - 1;
        let bit = (self.took_best_trail as u32) << 18;
        let val = (self.fountain & mask) | bit;
        val.hash(state);
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.fountain, self.took_best_trail)
    }
}
