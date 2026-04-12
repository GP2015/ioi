pub struct LF<'a> {
    data: &'a [i32],
}

impl<'a> LF<'a> {
    pub fn from(l: &'a [i32]) -> Self {
        Self { data: l }
    }

    pub fn get(&self, highway: u32) -> u32 {
        self.data[highway as usize] as u32
    }
}
