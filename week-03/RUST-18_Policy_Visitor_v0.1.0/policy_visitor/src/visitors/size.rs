use crate::{AccessLogs, KeyStorage, SecurityVisitor};
use std::cell::Cell;

/// Visitor que calcula el tamaño total sproximado.
#[derive(Default)]
pub struct SizeAuditor {
    total: Cell<usize>,
}

impl SizeAuditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total_bytes(&self) -> usize {
        self.total.get()
    }

    fn add(&self, amount: usize) {
        self.total.set(self.total.get() + amount);
    }
}

impl SecurityVisitor for SizeAuditor {
    fn visit_key_storage(&self, storage: &KeyStorage) {
        self.add(storage.size_bytes_estimate());
    }

    fn visit_logs(&self, logs: &AccessLogs) {
        self.add(logs.size_bytes_estimate());
    }
}
