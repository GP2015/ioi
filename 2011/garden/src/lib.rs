#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::missing_panics_doc,
    clippy::many_single_char_names,
    clippy::undocumented_unsafe_blocks
)]

mod graph;
mod p_info;
mod parse;
mod passed_map;

use crate::parse::ParsedArgs;
use core::ffi::c_int;
use mimalloc::MiMalloc;

unsafe extern "C" {
    safe fn answer(x: c_int);
}

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

struct RawArgs {
    pub n: c_int,
    pub m: c_int,
    pub p: c_int,
    pub r: *const c_int,
    pub q: c_int,
    pub g: *const c_int,
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
    let raw_args = RawArgs { n, m, p, r, q, g };
    let parsed_args = ParsedArgs::from_raw(&raw_args);
    count_routes_impl(&parsed_args);
}

fn count_routes_impl(args: &ParsedArgs) {
    let ParsedArgs { n, p, r, g } = *args;
    let graph = graph::create(n, r);

    let p_info = p_info::create(graph, n, p);

    //

    for &steps in g {
        let number_of_routes = (0..n)
            .filter(|&start_fount| state_reaches_p_in_steps(graph, start_fount, steps, p))
            .count();

        answer(number_of_routes.try_into().unwrap());
    }
}

fn state_reaches_p_in_steps(graph: &[u32], start_fount: u32, steps: u32, p: u32) -> bool {
    let mut idx = start_fount << 1;

    for _ in 0..steps {
        idx = graph[idx as usize];
    }

    idx >> 1 == p
}
