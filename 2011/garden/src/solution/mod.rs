mod pmap;
mod state;
mod state_map;

use crate::solution::state_map::StateMap;

pub fn count_routes_safe(n: u32, p: u32, r: &[[u32; 2]], g: &[u32]) {
    let state_map = StateMap::from(n, p, r);

    for &steps in g {
        let mut number_of_routes = 0;
        for start_fount in 0..n {
            if state_reaches_p_in_steps(&state_map, start_fount, steps, p) {
                number_of_routes += 1;
            }
        }

        crate::answer(number_of_routes);
    }
}

fn state_reaches_p_in_steps(state_map: &StateMap, start_fount: u32, steps: u32, p: u32) -> bool {
    let point = &state_map.runner[start_fount as usize];

    let Some(p_hit_info) = point.p_hit_info else {
        return false;
    };

    if steps < p_hit_info.steps_to {
        return false;
    }

    let p_point = &state_map.points(p_hit_info.best)[p as usize];

    let Some(p2_hit_info) = p_point.p_hit_info else {
        return steps == p_hit_info.steps_to;
    };

    if p_hit_info.best == p2_hit_info.best {
        let steps_dif = steps - p_hit_info.steps_to;
        return p2_hit_info.steps_to == 0 || steps_dif.is_multiple_of(p2_hit_info.steps_to);
    }

    if steps < p_hit_info.steps_to + p2_hit_info.steps_to {
        return steps == p_hit_info.steps_to;
    }

    let p2_point = &state_map.points(p2_hit_info.best)[p as usize];

    let Some(p3_hit_info) = p2_point.p_hit_info else {
        return steps == p_hit_info.steps_to || steps == p2_hit_info.steps_to;
    };

    if p2_hit_info.best == p3_hit_info.best {
        if steps == p_hit_info.steps_to {
            return true;
        }

        let steps_dif = steps - p_hit_info.steps_to - p2_hit_info.steps_to;
        return p3_hit_info.steps_to == 0 || steps_dif.is_multiple_of(p3_hit_info.steps_to);
    }

    let steps_to_loop = p2_hit_info.steps_to + p3_hit_info.steps_to;

    for steps_dif in [
        steps - p_hit_info.steps_to,
        steps - p_hit_info.steps_to - p2_hit_info.steps_to,
    ] {
        if steps_to_loop == 0 || steps_dif.is_multiple_of(steps_to_loop) {
            return true;
        }
    }

    false
}
