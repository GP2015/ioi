mod array_readers;
mod solution;
use crate::array_readers::{gf::GF, rf::RF};
use std::ffi::c_int;

#[cfg(feature = "par")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg(feature = "mem")]
#[global_allocator]
static PEAK_ALLOC: peak_alloc::PeakAlloc = peak_alloc::PeakAlloc;

#[cfg(not(feature = "mem"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

unsafe extern "C" {
    fn answer(x: c_int);
}

/// Implementation of the count_routes function.
///
/// # Safety
///
/// Behaviour is undefined is any of the following conditions are violated:
///
/// * `r` must point to an array that is twice as long as length `m`.
///
/// * `g` must point to an array of length `q`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_routes(
    n: c_int,
    m: c_int,
    p: c_int,
    r: *const c_int,
    q: c_int,
    g: *const c_int,
) {
    assert!((2..150_001).contains(&n));
    assert!((1..150_001).contains(&m));
    assert!((0..150_000).contains(&p));
    assert!((1..2_001).contains(&q));

    let r = unsafe { std::slice::from_raw_parts(r, m as usize * 2) };
    let g = unsafe { std::slice::from_raw_parts(g, q as usize) };

    #[cfg(not(feature = "par"))]
    {
        r.iter().for_each(|val| assert!((0..150_000).contains(val)));
        g.iter()
            .for_each(|val| assert!((1..1_000_000_001).contains(val)));
    }

    #[cfg(feature = "par")]
    {
        r.par_iter()
            .for_each(|val| assert!((0..150_000).contains(val)));
        g.par_iter()
            .for_each(|val| assert!((1..1_000_000_001).contains(val)));
    }

    let (r, _) = r.as_chunks::<2>();

    solution::count_routes_safe(
        n as u32,
        m as u32,
        p as u32,
        RF::from(r),
        q as u16,
        GF::from(g),
    );

    #[cfg(feature = "mem")]
    println!("Memory: {} MB", PEAK_ALLOC.peak_usage_as_mb());
}

fn call_answer(x: usize) {
    unsafe { answer(x as i32) };
}
