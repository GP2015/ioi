mod out;

use crate::solution::cities::single::out::HighwayOutInfo;

#[derive(Clone)]
pub struct CityInfo {
    outbound_routes: Vec<HighwayOutInfo>,
}

impl CityInfo {
    pub fn new() -> Self {
        Self {
            outbound_routes: Vec::new(),
        }
    }

    pub fn add(&mut self, highway: u32, highway_length: u32, next_city: u32) {
        let route = HighwayOutInfo::from(highway, highway_length, next_city);
        self.outbound_routes.push(route);
    }
}
