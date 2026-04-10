pub struct RF<'a> {
    data: &'a [[i32; 2]],
}

impl<'a> RF<'a> {
    pub fn from(r: &'a [[i32; 2]]) -> Self {
        Self { data: r }
    }

    pub fn get(&self, trail: u32, side: usize) -> u32 {
        self.data[trail as usize][side] as u32
    }
}
