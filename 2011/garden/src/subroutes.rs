use crate::{cache::Cache, fountain::Fountain, state::State, util};
use ahash::AHashMap;

pub const SUBROUTES_LENGTH: usize = 30;

pub fn generate_subroutes(
    fountains: &[Fountain],
    n: u32,
) -> [AHashMap<State, State>; SUBROUTES_LENGTH] {
    let mut subroutes = std::array::from_fn(|_| AHashMap::new());

    for fountain in 0..n {
        for took_best_trail in [true, false] {
            let start_state = State::from(fountain, took_best_trail);
            let outbound = fountains[fountain as usize].outbound(took_best_trail);
            let best_trail_next =
                fountains[outbound.fountain as usize].is_best_trail(outbound.trail);
            let next_state = State::from(outbound.fountain, best_trail_next);
            subroutes[0].insert(start_state, next_state);
        }
    }

    for exp in 1..SUBROUTES_LENGTH {
        for fountain in 0..n {
            for took_best_trail in [true, false] {
                let start_state = State::from(fountain, took_best_trail);
                let inter_state = subroutes[exp - 1].get(&start_state).unwrap();
                let end_state = subroutes[exp - 1].get(inter_state).unwrap();
                subroutes[exp].insert(start_state, *end_state);
            }
        }
    }

    subroutes
}

pub fn state_plus_steps(
    steps: u32,
    start_state: State,
    subroutes: &[AHashMap<State, State>; SUBROUTES_LENGTH],
    cache: &mut Cache,
) -> State {
    let key = (start_state, steps);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let prev_pow = util::prev_power_of_two(steps);
    let exp = prev_pow.ilog2() as usize;
    let next_state = subroutes[exp].get(&start_state).unwrap();

    let remaining_steps = steps - prev_pow;

    if remaining_steps == 0 {
        return *next_state;
    }

    let end_state = state_plus_steps(remaining_steps, *next_state, subroutes, cache);
    cache.insert(key, end_state);
    end_state
}
