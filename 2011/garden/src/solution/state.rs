#[derive(Clone, Copy)]
pub struct State {
    pub fount: u32,
    pub best: bool,
}

impl State {
    pub fn id(self) -> u32 {
        (self.fount << 1) | u32::from(self.best)
    }
}
