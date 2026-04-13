use std::hint;

use getset::CopyGetters;
use no_panic::no_panic;

#[derive(Clone, Copy, CopyGetters)]
pub struct State {
    #[getset(get_copy = "pub")]
    fountain: u32,
    #[getset(get_copy = "pub")]
    took_best_trail: bool,
}

impl State {
    #[no_panic]
    pub fn from(fountain: u32, took_best_trail: bool) -> Self {
        Self {
            fountain,
            took_best_trail,
        }
    }

    #[no_panic]
    pub fn id(self, n: u32) -> u32 {
        let id = (self.fountain << 1) | u32::from(self.took_best_trail);

        // Safety: id cannot be greater than n * 2
        unsafe {
            hint::assert_unchecked(id < n * 2);
        }

        id
    }
}
