mod state;
mod state_map;

use std::ffi::c_int;

use crate::state_map::StateMap;

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
    let mut state_map = StateMap::new(n);
    state_map.add_next_states(m, r);
    state_map.add_distances_to_p(n, p);

    for group in 0..q {
        let steps = g.get(group) as usize;
        let mut number_of_routes = 0;

        for starting_fountain in 0..n {
            let point = state_map.point(starting_fountain, false);

            let Some(steps_to_p) = point.steps_to_p else {
                continue;
            };

            if steps < steps_to_p {
                continue;
            }

            let best_trail_to_p = point.best_trail_to_p;

            let p_point = state_map.point(p, best_trail_to_p);

            let Some(steps_to_p2) = p_point.steps_to_p else {
                if steps == steps_to_p {
                    number_of_routes += 1;
                }
                continue;
            };

            let best_trail_to_p2 = p_point.best_trail_to_p;

            if best_trail_to_p == best_trail_to_p2 {
                let steps_dif = steps - steps_to_p;

                if steps_dif.is_multiple_of(steps_to_p2) || steps_to_p2 == 0 {
                    number_of_routes += 1;
                }
                continue;
            }

            if steps < steps_to_p + steps_to_p2 {
                if steps == steps_to_p {
                    number_of_routes += 1;
                }
                continue;
            }

            let p2_point = state_map.point(p, best_trail_to_p2);

            let Some(steps_to_p3) = p2_point.steps_to_p else {
                if steps == steps_to_p || steps == steps_to_p2 {
                    number_of_routes += 1;
                }
                continue;
            };

            let best_trail_to_p3 = p2_point.best_trail_to_p;

            if best_trail_to_p2 == best_trail_to_p3 {
                if steps == steps_to_p {
                    number_of_routes += 1;
                    continue;
                }

                let steps_dif = steps - steps_to_p - steps_to_p2;

                if steps_dif.is_multiple_of(steps_to_p3) || steps_to_p3 == 0 {
                    number_of_routes += 1;
                }
                continue;
            }

            let steps_to_loop = steps_to_p2 + steps_to_p3;

            for steps_dif in [steps - steps_to_p, steps - steps_to_p - steps_to_p2] {
                if steps_dif.is_multiple_of(steps_to_loop) || steps_to_loop == 0 {
                    number_of_routes += 1;
                    break;
                }
            }
        }

        call_answer(number_of_routes);
    }
}
