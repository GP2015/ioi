mod single;

use crate::solution::cities::single::CityInfo;

pub struct Cities {
    cities: Vec<CityInfo>,
}

impl Cities {
    pub fn from(n: u32, h: &[[u32; 2]], l: &[u32]) -> Self {
        let mut cities = vec![CityInfo::new(); n as usize];

        for (highway, (&[city1, city2], &highway_length)) in h.iter().zip(l.iter()).enumerate() {
            cities[city1 as usize].add(highway as u32, highway_length, city2);
            cities[city2 as usize].add(highway as u32, highway_length, city1);
        }

        Self { cities }
    }
}
