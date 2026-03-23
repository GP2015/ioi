#[derive(Copy, Clone)]
pub struct Outbound {
    pub trail: u32,
    pub fountain: u32,
}

impl Outbound {
    pub(super) fn from(trail: u32, fountain: u32) -> Self {
        Self { trail, fountain }
    }
}
