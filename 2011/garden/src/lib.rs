#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::missing_panics_doc,
    clippy::many_single_char_names,
    clippy::similar_names,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]

mod solution;

unsafe extern "C" {
    safe fn answer(x: c_int);
}

use std::{ffi::c_int, slice};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
    assert!((2..150_001).contains(&n));
    assert!((1..150_001).contains(&m));
    assert!((0..150_000).contains(&p));
    assert!((1..2_001).contains(&q));

    // Safety: r must point to an array that is twice as long as length m.
    let r = unsafe { slice::from_raw_parts(r.cast(), m as usize * 2) };
    let (r, _) = r.as_chunks::<2>();

    // Safety: g must point to an array of length q.
    let g = unsafe { slice::from_raw_parts(g.cast(), q as usize) };

    assert_eq!(r.len(), m as usize);
    assert_eq!(g.len(), q as usize);

    for row in r {
        assert!((0..150_000).contains(&row[0]));
        assert!((0..150_000).contains(&row[1]));
    }

    for val in g {
        assert!((1..1_000_000_001).contains(val));
    }

    solution::count_routes_safe(n as u32, p as u32, r, g);
}

fn call_answer(x: usize) {
    answer(x as i32);
}
