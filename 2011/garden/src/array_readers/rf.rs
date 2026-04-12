pub struct RF<'a> {
    data: &'a [[i32; 2]],
}

impl<'a> RF<'a> {
    #[no_panic::no_panic]
    pub fn from(r: &'a [[i32; 2]]) -> Self {
        Self { data: r }
    }

    // #[no_panic::no_panic]
    pub fn get(&self, trail: u32, side: bool) -> u32 {
        self.data[trail as usize][usize::from(side)] as u32
    }
}
