pub mod outbound;
pub mod single;

use crate::{RF, fountains::single::Fountain};

pub struct Fountains {
    data: Vec<Fountain>,
}

impl Fountains {
    pub fn from(n: u32, m: u32, r: &RF) -> Self {
        let mut fountains = Self {
            data: vec![Fountain::new(); n as usize],
        };

        for trail in 0..m {
            for side in [true, false] {
                let fountain = &mut fountains.data[r.get(trail, side) as usize];
                if fountain.can_add_next() {
                    fountain.add_outbound(trail, r.get(trail, !side));
                }
            }
        }

        fountains
    }

    pub fn get(&self, fountain: u32) -> &Fountain {
        &self.data[fountain as usize]
    }
}
