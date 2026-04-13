pub struct RF<'a> {
    data: &'a [[i32; 2]],
}

impl<'a> RF<'a> {
    pub fn from(r: &'a [[i32; 2]], m: usize) -> Self {
        assert!(r.len() == m);
        for row in r {
            assert!((0..150_000).contains(&row[0]));
            assert!((0..150_000).contains(&row[1]));
        }
        Self { data: r }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, u32)> {
        self.data.iter().map(|row| (row[0] as u32, row[1] as u32))
    }
}
