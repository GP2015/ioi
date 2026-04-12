pub struct HF<'a> {
    data: &'a [[i32; 2]],
}

impl<'a> HF<'a> {
    pub fn from(h: &'a [[i32; 2]]) -> Self {
        Self { data: h }
    }

    pub fn get(&self, highway: u32, side: usize) -> u32 {
        self.data[highway as usize][side] as u32
    }
}
