mod array_readers;
mod solution;

use std::ffi::c_int;

#[cfg(feature = "mem")]
use peak_alloc::PeakAlloc;

use crate::array_readers::{hf::HF, lf::LF};

#[cfg(feature = "mem")]
#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

/// Implementation of the best_path function.
///
/// # Safety
///
/// Behaviour is undefined is any of the following conditions are violated:
///
/// * `h` must point to an array that is twice as long as length `n` minus 1.
///
/// * `l` must point to an array of length `n`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn best_path(n: c_int, k: c_int, h: *const c_int, l: *const c_int) -> c_int {
    assert!((1..=200_000).contains(&n));
    assert!((1..=1_000_000).contains(&k));

    let h = unsafe { std::slice::from_raw_parts(h, (n as usize - 1) * 2) };
    h.iter().for_each(|val| assert!((0..200_000).contains(val)));
    let (h, _) = h.as_chunks::<2>();

    let l = unsafe { std::slice::from_raw_parts(l, n as usize) };
    l.iter()
        .for_each(|val| assert!((0..=1_000_000_000).contains(val)));

    let ans = solution::best_path_safe(n as u32, k as u32, HF::from(h), LF::from(l));

    #[cfg(feature = "mem")]
    println!("Memory: {} mb", PEAK_ALLOC.peak_usage_as_mb());

    ans
}
