pub struct RF<'a> {
    data: &'a [[i32; 2]],
}

impl<'a> RF<'a> {
    pub fn from(r: &'a [[i32; 2]]) -> Self {
        Self { data: r }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, u32)> {
        self.data.iter().map(|row| (row[0] as u32, row[1] as u32))
    }
}
