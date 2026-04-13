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

use std::{ffi::c_int, hint};

#[cfg(feature = "mem")]
#[global_allocator]
static PEAK_ALLOC: peak_alloc::PeakAlloc = peak_alloc::PeakAlloc;

#[cfg(not(feature = "mem"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Implementation of the `best_path` function.
///
/// # Safety
///
/// Behaviour is undefined if the parameters do not uphold the specification.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn best_path(n: c_int, k: c_int, h: *const c_int, l: *const c_int) -> c_int {
    // Safety: n and m must uphold the specification.
    unsafe {
        hint::assert_unchecked((1..200_001).contains(&n));
        hint::assert_unchecked((1..1_000_001).contains(&k));
    }

    // Safety: h must point to an array that is twice as long as length n minus 1.
    let h = unsafe { std::slice::from_raw_parts(h.cast(), (n as usize - 1) * 2) };
    let (h, _) = h.as_chunks::<2>();

    // Safety: `l` must point to an array of length `n`.
    let l = unsafe { std::slice::from_raw_parts(l.cast(), n as usize) };

    // Safety: h and l must be the correct size.
    unsafe {
        hint::assert_unchecked(h.len() == n as usize - 1);
        hint::assert_unchecked(l.len() == n as usize);
    }

    // Safety: The values at r and g must uphold the specification.
    unsafe {
        for row in h {
            hint::assert_unchecked((0..200_000).contains(&row[0]));
            hint::assert_unchecked((0..200_000).contains(&row[1]));
        }

        for val in l {
            hint::assert_unchecked((0..1_000_000_001).contains(val));
        }
    }

    let ans = solution::best_path_safe(n as u32, k as u32, h, l);

    #[cfg(feature = "mem")]
    println!("Memory: {} mb", PEAK_ALLOC.peak_usage_as_mb());

    ans
}
