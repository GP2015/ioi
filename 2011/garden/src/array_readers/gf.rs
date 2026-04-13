pub struct GF<'a> {
    data: &'a [i32],
}

impl<'a> GF<'a> {
    pub fn from(r: &'a [i32]) -> Self {
        Self { data: r }
    }

    pub fn get(&self, group: u16) -> u32 {
        self.data[group as usize] as u32
    }
}
