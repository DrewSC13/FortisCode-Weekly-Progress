use crate::{DecryptionService, RealVault};
use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};

/// Proxy: AuditProxy
/// - Contiene una instancia de `RealVault`
/// - Contiene un registro de logs (`Vec<String>`)
/// - Controla tamaño sospechoso antes de delegar
pub struct AuditProxy {
    vault: RealVault,
    logs: RefCell<Vec<String>>,
    max_len: usize,
}

impl AuditProxy {
    pub fn new(vault: RealVault) -> Self {
        Self {
            vault,
            logs: RefCell::new(Vec::new()),
            max_len: 1024,
        }
    }

    pub fn with_max_len(mut self, max_len: usize) -> Self {
        self.max_len = max_len;
        self
    }

    pub fn logs(&self) -> Vec<String> {
        self.logs.borrow().clone()
    }

    fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

impl DecryptionService for AuditProxy {
    fn decrypt(&self, data: Vec<u8>) -> String {
        let ts = Self::timestamp();

        // 1) Registrar intento SIEMPRE
        self.logs
            .borrow_mut()
            .push(format!("Intento de descifrado a las {}", ts));

        // 2) Verificar tamaño sospechoso y bloquear
        if data.len() > self.max_len {
            self.logs.borrow_mut().push(format!(
                "Fallo de descifrado a las {ts}: payload demasiado grande (len={})",
                data.len()
            ));
            return "BLOCKED".to_string();
        }

        // 3) Delegar al servicio real
        self.vault.decrypt(data)
    }
}
