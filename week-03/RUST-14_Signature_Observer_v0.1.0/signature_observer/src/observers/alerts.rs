use crate::SecurityObserver;
use std::cell::Cell;

/// AlertSystem: cambia un flag de "Alerta Roja".
pub struct AlertSystem {
    red_alert: Cell<bool>,
}

impl AlertSystem {
    pub fn new() -> Self {
        Self {
            red_alert: Cell::new(false),
        }
    }

    pub fn is_red_alert(&self) -> bool {
        self.red_alert.get()
    }
}

impl Default for AlertSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityObserver for AlertSystem {
    fn update(&self, _event_details: &str) {
        self.red_alert.set(true);
    }
}
