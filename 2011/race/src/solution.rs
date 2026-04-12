mod cities;

use crate::{
    array_readers::{hf::HF, lf::LF},
    solution::cities::Cities,
};

pub fn best_path_safe(n: u32, k: u32, h: HF, l: LF) -> i32 {
    let cities = Cities::from(n, h, l);

    for start_city in 0..n {
        let used_highways = 1;
    }

    k as i32
}
