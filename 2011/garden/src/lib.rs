mod check;
mod passed_map;
mod state;
mod state_map;

use crate::state_map::StateMap;
use std::ffi::c_int;

#[cfg(feature = "par")]
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[cfg(feature = "memtrack")]
#[global_allocator]
static PEAK_ALLOC: peak_alloc::PeakAlloc = peak_alloc::PeakAlloc;

#[cfg(not(feature = "memtrack"))]
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

    count_routes_safe(
        n as u32,
        m as u32,
        p as u32,
        RF::from(r),
        q as u16,
        GF::from(g),
    );

    #[cfg(feature = "memtrack")]
    println!("Memory: {} MB", PEAK_ALLOC.peak_usage_as_mb());
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

    fn get(&self, trail: u32, side: usize) -> u32 {
        self.data[trail as usize][side] as u32
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
    let state_map = StateMap::from(n, m, p, r);
    solve(state_map, n, p, q, g);
}

#[cfg(not(feature = "par"))]
fn solve(state_map: StateMap, n: u32, p: u32, q: u16, g: GF) {
    for group in 0..q {
        let steps = g.get(group);

        let mut number_of_routes = 0;
        for starting_fountain in 0..n {
            if state_reaches_p_in_steps(&state_map, starting_fountain, steps, p) {
                number_of_routes += 1;
            }
        }

        call_answer(number_of_routes);
    }
}

#[cfg(feature = "par")]
fn solve(state_map: StateMap, n: u32, p: u32, q: u16, g: GF) {
    (0..q)
        .into_par_iter()
        .map(|group| {
            let steps = g.get(group);
            (0..n)
                .into_par_iter()
                .filter(|&starting_fountain| {
                    state_reaches_p_in_steps(&state_map, starting_fountain, steps, p)
                })
                .count()
        })
        .collect::<Vec<usize>>()
        .into_iter()
        .for_each(call_answer);
}

fn state_reaches_p_in_steps(
    state_map: &StateMap,
    starting_fountain: u32,
    steps: u32,
    p: u32,
) -> bool {
    let point = state_map.point(starting_fountain, false);

    let Some((steps_to_p, p_took_best_trail)) = point.p_hit_info() else {
        return false;
    };

    if steps < steps_to_p {
        return false;
    }

    let p_point = state_map.point(p, p_took_best_trail);

    let Some((steps_to_p2, p2_took_best_trail)) = p_point.p_hit_info() else {
        return steps == steps_to_p;
    };

    if p_took_best_trail == p2_took_best_trail {
        let steps_dif = steps - steps_to_p;
        return steps_dif.is_multiple_of(steps_to_p2) || steps_to_p2 == 0;
    }

    if steps < steps_to_p + steps_to_p2 {
        return steps == steps_to_p;
    }

    let p2_point = state_map.point(p, p2_took_best_trail);

    let Some((steps_to_p3, p3_took_best_trail)) = p2_point.p_hit_info() else {
        return steps == steps_to_p || steps == steps_to_p2;
    };

    if p2_took_best_trail == p3_took_best_trail {
        if steps == steps_to_p {
            return true;
        }

        let steps_dif = steps - steps_to_p - steps_to_p2;
        return steps_dif.is_multiple_of(steps_to_p3) || steps_to_p3 == 0;
    }

    let steps_to_loop = steps_to_p2 + steps_to_p3;

    for steps_dif in [steps - steps_to_p, steps - steps_to_p - steps_to_p2] {
        if steps_dif.is_multiple_of(steps_to_loop) || steps_to_loop == 0 {
            return true;
        }
    }

    false
}
