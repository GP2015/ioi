mod passed_map;
mod state;
mod state_map;

use crate::{
    array_readers::{gf::GF, rf::RF},
    solution::state_map::StateMap,
};
#[cfg(feature = "par")]
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};

#[cfg(not(feature = "par"))]
pub fn count_routes_safe(n: u32, p: u32, r: &RF, g: &GF) {
    let state_map = StateMap::from(n, p, r);

    for steps in g.iter() {
        let mut number_of_routes = 0;
        for starting_fountain in 0..state_map.n() {
            if state_reaches_p_in_steps(&state_map, starting_fountain, steps, p) {
                number_of_routes += 1;
            }
        }

        crate::call_answer(number_of_routes);
    }
}

#[cfg(feature = "par")]
pub fn count_routes_safe(n: u32, p: u32, r: &RF, g: &GF) {
    let state_map = StateMap::from(n, p, r);

    g.par_iter()
        .map(|steps| {
            (0..state_map.n())
                .into_par_iter()
                .filter(|&starting_fountain| {
                    state_reaches_p_in_steps(&state_map, starting_fountain, steps, p)
                })
                .count()
        })
        .collect::<Vec<usize>>()
        .into_iter()
        .for_each(crate::call_answer);
}

fn state_reaches_p_in_steps(
    state_map: &StateMap,
    starting_fountain: u32,
    steps: u32,
    p: u32,
) -> bool {
    let point = state_map.point(starting_fountain, false);

    let Some(p_hit_info) = point.p_hit_info() else {
        return false;
    };

    if steps < p_hit_info.steps_to() {
        return false;
    }

    let p_point = state_map.point(p, p_hit_info.took_best_trail());

    let Some(p2_hit_info) = p_point.p_hit_info() else {
        return steps == p_hit_info.steps_to();
    };

    if p_hit_info.took_best_trail() == p2_hit_info.took_best_trail() {
        let steps_dif = steps - p_hit_info.steps_to();
        return steps_dif.is_multiple_of(p2_hit_info.steps_to()) || p2_hit_info.steps_to() == 0;
    }

    if steps < p_hit_info.steps_to() + p2_hit_info.steps_to() {
        return steps == p_hit_info.steps_to();
    }

    let p2_point = state_map.point(p, p2_hit_info.took_best_trail());

    let Some(p3_hit_info) = p2_point.p_hit_info() else {
        return steps == p_hit_info.steps_to() || steps == p2_hit_info.steps_to();
    };

    if p2_hit_info.took_best_trail() == p3_hit_info.took_best_trail() {
        if steps == p_hit_info.steps_to() {
            return true;
        }

        let steps_dif = steps - p_hit_info.steps_to() - p2_hit_info.steps_to();
        return steps_dif.is_multiple_of(p3_hit_info.steps_to()) || p3_hit_info.steps_to() == 0;
    }

    let steps_to_loop = p2_hit_info.steps_to() + p3_hit_info.steps_to();

    for steps_dif in [
        steps - p_hit_info.steps_to(),
        steps - p_hit_info.steps_to() - p2_hit_info.steps_to(),
    ] {
        if steps_dif.is_multiple_of(steps_to_loop) || steps_to_loop == 0 {
            return true;
        }
    }

    false
}
