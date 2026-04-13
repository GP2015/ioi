#![warn(
    clippy::pedantic,
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]
#![allow(
    clippy::many_single_char_names,
    clippy::similar_names,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]

mod solution;

use no_panic::no_panic;
use std::{ffi::c_int, hint, slice};

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
/// Behaviour is undefined if the parameters do not uphold the specification.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_routes(
    n: c_int,
    m: c_int,
    p: c_int,
    r: *const c_int,
    q: c_int,
    g: *const c_int,
) {
    // Safety: n, m, p, and q must uphold the specification.
    unsafe {
        hint::assert_unchecked((2..150_001).contains(&n));
        hint::assert_unchecked((1..150_001).contains(&m));
        hint::assert_unchecked((0..150_000).contains(&p));
        hint::assert_unchecked((1..2_001).contains(&q));
    }

    // Safety: r must point to an array that is twice as long as length m.
    let r = unsafe { slice::from_raw_parts(r.cast(), m as usize * 2) };
    let (r, _) = r.as_chunks::<2>();

    // Safety: g must point to an array of length q.
    let g = unsafe { slice::from_raw_parts(g.cast(), q as usize) };

    // Safety: r and g must be the same size.
    unsafe {
        hint::assert_unchecked(r.len() == m as usize);
        hint::assert_unchecked(g.len() == q as usize);
    }

    // Safety: The values at r and g must uphold the specification.
    unsafe {
        for row in r {
            hint::assert_unchecked((0..150_000).contains(&row[0]));
            hint::assert_unchecked((0..150_000).contains(&row[1]));
        }

        for val in g {
            hint::assert_unchecked((1..1_000_000_001).contains(val));
        }
    }

    solution::count_routes_safe(n as u32, p as u32, r, g);

    #[cfg(feature = "mem")]
    println!("Memory: {} MB", PEAK_ALLOC.peak_usage_as_mb());
}

#[no_panic]
fn call_answer(x: usize) {
    answer(x as i32);
}
