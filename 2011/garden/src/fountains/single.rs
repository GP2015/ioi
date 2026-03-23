use crate::fountains::outbound::Outbound;

#[derive(Clone)]
pub struct Fountain {
    best_next: Option<Outbound>,
    runner_next: Option<Outbound>,
}

impl Fountain {
    pub fn is_best_trail(&self, trail: u32) -> bool {
        let best_trail = self
            .best_next
            .expect(concat!(
                "best_next will always be filled before this method is called; ",
                "every fountain has at least one connecting trail"
            ))
            .trail;
        trail == best_trail
    }

    pub fn outbound(&self, took_best_trail: bool) -> Outbound {
        let best_next = self.best_next.expect(concat!(
            "best_next will always be filled before this method is called; ",
            "every fountain has at least one connecting trail"
        ));

        if took_best_trail && let Some(runner_next) = self.runner_next {
            runner_next
        } else {
            best_next
        }
    }

    pub(super) fn new() -> Self {
        Self {
            best_next: None,
            runner_next: None,
        }
    }

    pub(super) fn can_add_next(&self) -> bool {
        self.runner_next.is_none()
    }

    pub(super) fn add_outbound(&mut self, trail: u32, fountain: u32) {
        if self.best_next.is_none() {
            self.best_next = Some(Outbound::from(trail, fountain));
        } else {
            self.runner_next = Some(Outbound::from(trail, fountain));
        }
    }
}
