use crate::SecurityObserver;
use std::cell::RefCell;

/// AuditLogger: Escribe el fallo de un vector de strings.
/// Usa interior mutability para permitir update(&self, ...) y matar el buffer.
pub struct AuditLogger {
    buffer: RefCell<Vec<String>>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            buffer: RefCell::new(Vec::new()),
        }
    }

    pub fn logs(&self) -> Vec<String> {
        self.buffer.borrow().clone()
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityObserver for AuditLogger {
    fn update(&self, event_details: &str) {
        self.buffer.borrow_mut().push(event_details.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unitarias: Verificar que le AuditLogger almacena correctamente el mensaje recibido.
    #[test]
    fn audit_logger_stores_message() {
        let logger = AuditLogger::new();
        logger.update("firma invalida detectada");
        let logs = logger.logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "firma invalida detectada");
    }
}
