#[cfg(feature = "par")]
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator};

pub struct GF<'a> {
    data: &'a [i32],
}

impl<'a> GF<'a> {
    pub fn from(g: &'a [i32], q: usize) -> Self {
        assert!(g.len() == q);
        for val in g {
            assert!((1..1_000_000_001).contains(val));
        }
        Self { data: g }
    }

    #[cfg(not(feature = "par"))]
    pub fn iter(&self) -> impl Iterator<Item = u32> {
        self.data.iter().map(|val| *val as u32)
    }

    #[cfg(feature = "par")]
    pub fn par_iter(&self) -> impl ParallelIterator<Item = u32> {
        self.data.par_iter().map(|val| *val as u32)
    }
}
