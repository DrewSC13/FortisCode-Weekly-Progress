use std::cell::RefCell;

use crate::{AccessLogs, KeyStorage, SecurityVisitor};

/// Visitor que detecta claves expiradas.
/// Política: expirada si `expires_year < current_year`.
pub struct ExpirationChecker {
    current_year: u32,
    expired: RefCell<Vec<String>>,
}

impl ExpirationChecker {
    pub fn new(current_year: u32) -> Self {
        Self {
            current_year,
            expired: RefCell::new(Vec::new()),
        }
    }

    pub fn expired_ids(&self) -> Vec<String> {
        self.expired.borrow().clone()
    }

    fn mark_expired(&self, id: &str) {
        self.expired.borrow_mut().push(id.to_string());
    }
}

impl SecurityVisitor for ExpirationChecker {
    fn visit_key_storage(&self, storage: &KeyStorage) {
        for key in storage.keys() {
            if key.expires_year < self.current_year {
                self.mark_expired(&key.id);
            }
        }
    }

    fn visit_logs(&self, _logs: &AccessLogs) {
        // No aplica para expiración.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{KeyRecord, KeyStorage, SecurityElement};

    #[test]
    fn expiration_checker_flags_2020_key_as_expired() {
        let storage = KeyStorage::new(vec![
            KeyRecord::new("key-2020", 2020),
            KeyRecord::new("key-2030", 2030),
        ]);

        let checker = ExpirationChecker::new(2026);
        storage.accept(&checker);

        let expired = checker.expired_ids();
        assert!(expired.contains(&"key-2020".to_string()));
        assert!(!expired.contains(&"key-2030".to_string()));
    }
}
