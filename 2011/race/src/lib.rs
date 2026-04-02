use std::ffi::c_int;

/// Implementation of the best_path function.
///
/// # Safety
///
/// Behaviour is undefined is any of the following conditions are violated:
///
/// * `h` must point to an array that is twice as long as length `n`.
///
/// * `l` must point to an array of length `n`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn best_path(n: c_int, k: c_int, h: *const c_int, l: *const c_int) -> c_int {
    assert!((1..=200_000).contains(&n));
    assert!((1..=1_000_000).contains(&k));

    let h = unsafe { std::slice::from_raw_parts(h, (n * 2) as usize) };
    h.iter().for_each(|val| assert!((0..200_000).contains(val)));
    let (h, _) = h.as_chunks::<2>();

    let l = unsafe { std::slice::from_raw_parts(l, n as usize) };
    l.iter()
        .for_each(|val| assert!((0..=1_000_000_000).contains(val)));

    best_path_safe(n as u32, k as u32, HF::from(h), LF::from(l))
}

struct HF<'a> {
    data: &'a [[i32; 2]],
}

impl<'a> HF<'a> {
    fn from(r: &'a [[i32; 2]]) -> Self {
        Self { data: r }
    }

    fn get(&self, trail: u32, side: usize) -> u32 {
        self.data[trail as usize][side] as u32
    }
}

struct LF<'a> {
    data: &'a [i32],
}

impl<'a> LF<'a> {
    fn from(r: &'a [i32]) -> Self {
        Self { data: r }
    }

    fn get(&self, highway: u32) -> u32 {
        self.data[highway as usize] as u32
    }
}

fn best_path_safe(n: u32, k: u32, h: HF, l: LF) -> i32 {
    n as i32
}
