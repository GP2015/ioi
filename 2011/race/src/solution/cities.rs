mod single;

use crate::{HF, LF, solution::cities::single::CityInfo};

pub struct Cities {
    cities: Vec<CityInfo>,
}

impl Cities {
    pub fn from(n: u32, h: HF, l: LF) -> Self {
        let mut cities = vec![CityInfo::new(); n as usize];

        for highway in 0..(n - 1) {
            for side in 0..2 {
                let current_city = h.get(highway, side);
                let next_city = h.get(highway, 1 - side);
                let highway_length = l.get(highway);
                cities[current_city as usize].add(highway, highway_length, next_city);
            }
        }

        Self { cities }
    }
}
