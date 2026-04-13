#![warn(clippy::pedantic)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

mod array_readers;
mod solution;

use crate::array_readers::{gf::GF, rf::RF};
use core::{ffi::c_int, slice};

#[cfg(feature = "mem")]
#[global_allocator]
static PEAK_ALLOC: peak_alloc::PeakAlloc = peak_alloc::PeakAlloc;

#[cfg(not(feature = "mem"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

unsafe extern "C" {
    safe fn answer(x: c_int);
}

/// Implementation of the `count_routes` function.
///
/// # Safety
///
/// Behaviour is undefined is any of the following conditions are violated:
///
/// * `r` must point to an array that is twice as long as length `m`.
///
/// * `g` must point to an array of length `q`.
///
/// # Panics
///
/// May panic if the provided input data does not follow the specification.
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

    // Safety: `r` must point to an array that is twice as long as length `m`.
    let r = unsafe { slice::from_raw_parts(r, m as usize * 2) };

    // Safety: `g` must point to an array of length `q`.
    let g = unsafe { slice::from_raw_parts(g, q as usize) };

    for val in r {
        assert!((0..150_000).contains(val));
    }

    for val in g {
        assert!((1..1_000_000_001).contains(val));
    }

    let (r, _) = r.as_chunks::<2>();

    solution::count_routes_safe(
        n as u32,
        m as u32,
        p as u32,
        &RF::from(r),
        q as u16,
        &GF::from(g),
    );

    #[cfg(feature = "mem")]
    println!("Memory: {} MB", PEAK_ALLOC.peak_usage_as_mb());
}

fn call_answer(x: usize) {
    answer(x as i32);
}
