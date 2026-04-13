use getset::CopyGetters;
use no_panic::no_panic;

#[derive(Clone, Copy, CopyGetters)]
pub struct HighwayOutInfo {
    #[getset(get_copy = "pub")]
    highway: u32,
    #[getset(get_copy = "pub")]
    highway_length: u32,
    #[getset(get_copy = "pub")]
    next_city: u32,
}

impl HighwayOutInfo {
    #[no_panic]
    pub fn from(highway: u32, highway_length: u32, next_city: u32) -> Self {
        Self {
            highway,
            highway_length,
            next_city,
        }
    }
}
