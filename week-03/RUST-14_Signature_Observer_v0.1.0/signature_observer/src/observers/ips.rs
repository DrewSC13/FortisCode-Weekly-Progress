use crate::SecurityObserver;
use std::cell::RefCell;

/// IntrusionPreventer: Añade la IP a una lista negra.
pub struct IntrusionPreventer {
    blacklist: RefCell<Vec<String>>,
}

impl IntrusionPreventer {
    pub fn new() -> Self {
        Self {
            blacklist: RefCell::new(Vec::new()),
        }
    }

    pub fn blacklist(&self) -> Vec<String> {
        self.blacklist.borrow().clone()
    }

    fn extract_ip(details: &str) -> Option<String> {
        details
            .split_whitespace()
            .find_map(|part| part.strip_prefix("ip=").map(|ip| ip.to_string()))
    }
}

impl Default for IntrusionPreventer {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityObserver for IntrusionPreventer {
    fn update(&self, event_details: &str) {
        if let Some(ip) = Self::extract_ip(event_details) {
            self.blacklist.borrow_mut().push(ip);
        }
    }
}
