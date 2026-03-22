use crate::RF;

#[derive(Copy, Clone)]
pub struct Outbound {
    pub trail: u32,
    pub fountain: u32,
}

impl Outbound {
    fn from(trail: u32, fountain: u32) -> Self {
        Self { trail, fountain }
    }
}

#[derive(Clone)]
pub struct Fountain {
    pub best_next: Option<Outbound>,
    pub runner_next: Option<Outbound>,
}

impl Fountain {
    pub fn new() -> Self {
        Self {
            best_next: None,
            runner_next: None,
        }
    }

    pub fn is_best_trail(&self, trail: u32) -> bool {
        trail == self.best_next.unwrap().trail
    }

    pub fn can_add_next(&self) -> bool {
        self.runner_next.is_none()
    }

    pub fn add_outbound(&mut self, trail: u32, fountain: u32) {
        if self.best_next.is_none() {
            self.best_next = Some(Outbound::from(trail, fountain));
        } else {
            self.runner_next = Some(Outbound::from(trail, fountain));
        }
    }

    pub fn outbound(&self, took_best_trail: bool) -> Outbound {
        let best_next = self.best_next.unwrap();

        if took_best_trail && let Some(runner_next) = self.runner_next {
            runner_next
        } else {
            best_next
        }
    }
}

pub fn generate_fountains(n: u32, m: u32, r: RF) -> Vec<Fountain> {
    let mut fountains = vec![Fountain::new(); n as usize];

    for trail in 0..m {
        let fountain = &mut fountains[r.get(trail, false) as usize];
        if fountain.can_add_next() {
            fountain.add_outbound(trail, r.get(trail, true));
        }

        let fountain = &mut fountains[r.get(trail, true) as usize];
        if fountain.can_add_next() {
            fountain.add_outbound(trail, r.get(trail, false));
        }
    }

    fountains
}
