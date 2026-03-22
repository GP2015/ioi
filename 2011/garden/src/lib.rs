mod cache;
mod fountain;
mod state;
mod subroutes;
mod util;

use state::State;
use std::ffi::c_int;

use crate::cache::Cache;

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
///
/// * `n`, `m`, `p`, and `q`cannot be null.
///
/// * The arrays at `r` and `g` cannot contain null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_routes(
    n: c_int,
    m: c_int,
    p: c_int,
    r: *const c_int,
    q: c_int,
    g: *const c_int,
) {
    let r = unsafe { std::slice::from_raw_parts(r, (m * 2) as usize) };
    let (r, _) = r.as_chunks::<2>();

    let g = unsafe { std::slice::from_raw_parts(g, q as usize) };

    count_routes_safe(
        n as u32,
        m as u32,
        p as u32,
        RF::from(r),
        q as u16,
        GF::from(g),
    );
}

fn call_answer(x: usize) {
    unsafe { answer(x as i32) };
}

struct RF<'a> {
    data: &'a [[i32; 2]],
}

impl<'a> RF<'a> {
    fn from(r: &'a [[i32; 2]]) -> Self {
        Self { data: r }
    }

    fn get(&self, trail: u32, side: bool) -> u32 {
        self.data[trail as usize][side as usize] as u32
    }
}

struct GF<'a> {
    data: &'a [i32],
}

impl<'a> GF<'a> {
    fn from(r: &'a [i32]) -> Self {
        Self { data: r }
    }

    fn get(&self, group: u16) -> u32 {
        self.data[group as usize] as u32
    }
}

fn count_routes_safe(n: u32, m: u32, p: u32, r: RF, q: u16, g: GF) {
    // println!("n: {n}, m: {m}, q: {q}");

    // println!("Start");

    let fountains = fountain::generate_fountains(n, m, r);

    // println!("Fountains generated");

    let subroutes = subroutes::generate_subroutes(&fountains, n);

    // println!("Subroutes generated");

    let mut cache = Cache::new();

    for group in 0..q {
        let mut number_of_routes = 0;

        for start_fountain in 0..n {
            let start_state = State::from(start_fountain, false);

            let steps = g.get(group);
            let end_state = subroutes::state_plus_steps(steps, start_state, &subroutes, &mut cache);

            if end_state.fountain == p {
                number_of_routes += 1;
            }
        }

        call_answer(number_of_routes);
    }

    // cache.print_summary();

    // println!("Finished");
}
