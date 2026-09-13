use core::slice;
use std::hint;

use crate::RawArgs;

pub struct ParsedArgs {
    pub n: u32,
    pub p: u32,
    pub r: &'static [[u32; 2]],
    pub g: &'static [u32],
}

impl ParsedArgs {
    pub fn from_raw(raw: &RawArgs) -> Self {
        let RawArgs { n, m, p, r, q, g } = *raw;

        let m = m.try_into().unwrap();
        let q = q.try_into().unwrap();

        let parsed = Self {
            n: n.try_into().unwrap(),
            p: p.try_into().unwrap(),
            r: unsafe { slice::from_raw_parts(r.cast(), m * 2).as_chunks::<2>().0 },
            g: unsafe { slice::from_raw_parts(g.cast(), q) },
        };

        assert((2..150_001).contains(&parsed.n));
        assert((1..150_001).contains(&m));
        assert((0..150_000).contains(&parsed.p));
        assert((1..2_001).contains(&q));

        assert(parsed.r.len() == m);
        assert(parsed.g.len() == q);

        for row in parsed.r {
            assert((0..150_000).contains(&row[0]));
            assert((0..150_000).contains(&row[1]));
        }

        for val in parsed.g {
            assert((1..1_000_000_001).contains(val));
        }

        parsed
    }
}

fn assert(cond: bool) {
    // SAFETY: Data must follow spec
    unsafe {
        hint::assert_unchecked(cond);
    }
}
