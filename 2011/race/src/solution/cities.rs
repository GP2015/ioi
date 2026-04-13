mod single;

use crate::solution::cities::single::CityInfo;

pub struct Cities {
    cities: Vec<CityInfo>,
}

impl Cities {
    pub fn from(n: u32, h: &[[u32; 2]], l: &[u32]) -> Self {
        let mut cities = vec![CityInfo::new(); n as usize];

        for highway in 0..(n - 1) {
            for side in 0..2 {
                let current_city = h[highway as usize][side];
                let next_city = h[highway as usize][1 - side];
                let highway_length = l[highway as usize];
                cities[current_city as usize].add(highway, highway_length, next_city);
            }
        }

        Self { cities }
    }
}
