//! Tipo moderno: SecureData (Vec<u8> + metadatos de seguridad).
//! Requisito: Validar limpieza de memoria al destruir (Drop).

use std::sync::atomic::{Ordering, compiler_fence};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
}

/// Estructura moderna que mantiene datos sensibles.
/// - expone solo lectura (`as_slice`)
/// - en Drop limpia best-effort el buffer
pub struct SecureData {
    buffer: Vec<u8>,
    level: SecurityLevel,
    label: String,

    // TEST-ONLY: Permite verificar que el wipe se ejecuto
    #[cfg(test)]
    wipe_witness: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}

impl SecureData {
    pub fn new(buffer: Vec<u8>, level: SecurityLevel, label: impl Into<String>) -> Self {
        Self {
            buffer,
            level,
            label: label.into(),
            #[cfg(test)]
            wipe_witness: None,
        }
    }

    pub fn level(&self) -> SecurityLevel {
        self.level
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Acceso seguro (solo lectura). Sin exponer mutabilidad
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }

    /// (Crate-internal) Interop controlada: puntero mas tamanio para legacy.
    pub(crate) fn as_ptr_len(&self) -> (*const u8, usize) {
        (self.buffer.as_ptr(), self.buffer.len())
    }

    fn wipe_best_effort(&mut self) {
        // Limpieza best-effort
        for b in &mut self.buffer {
            unsafe { core::ptr::write_volatile(b, 0u8) };
        }
        compiler_fence(Ordering::SeqCst);

        #[cfg(test)]
        if let Some(w) = &self.wipe_witness {
            w.store(true, Ordering::SeqCst);
        }
    }

    #[cfg(test)]
    pub(crate) fn with_wipe_witness(
        mut self,
        witness: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        self.wipe_witness = Some(witness);
        self
    }
}

impl Drop for SecureData {
    fn drop(&mut self) {
        self.wipe_best_effort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    #[test]
    fn exposes_metadata_and_readonly_access() {
        let s = SecureData::new(vec![1, 2, 3], SecurityLevel::High, "vault-key");
        assert_eq!(s.level(), SecurityLevel::High);
        assert_eq!(s.label(), "vault-key");
        assert_eq!(s.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn drop_triggers_wipe_path() {
        let witness = Arc::new(AtomicBool::new(false));
        {
            let _s = SecureData::new(b"secret".to_vec(), SecurityLevel::Medium, "session-token")
                .with_wipe_witness(Arc::clone(&witness));
        }
        assert!(witness.load(Ordering::SeqCst));
    }
}
