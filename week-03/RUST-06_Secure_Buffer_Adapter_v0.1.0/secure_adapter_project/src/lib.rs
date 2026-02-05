//! Interfaz del Adaptador (API pública del crate).
//!
//! RUST-06 — Secure Buffer Adapter
//!
//! Objetivo:
//! - Exponer una API segura para integrar datos modernos (`SecureData`)
//!   con una librería legacy estilo C que usa punteros crudos.
//! - Encapsular completamente el uso de `unsafe` dentro del Adapter.
//! - Evitar exponer detalles de bajo nivel al usuario del crate.

pub mod legacy_api;
pub mod secure_type;

use core::fmt;
use legacy_api::legacy_crypto;

pub use secure_type::{SecureData, SecurityLevel};

/// Errores del Adapter (capa moderna).
#[derive(Debug)]
pub enum SecureAdapterError {
    EmptyInput,
    Legacy(legacy_crypto::LegacyCryptoError),
}

impl fmt::Display for SecureAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecureAdapterError::EmptyInput => write!(f, "secure data is empty"),
            SecureAdapterError::Legacy(e) => write!(f, "legacy error: {e}"),
        }
    }
}

impl std::error::Error for SecureAdapterError {}

impl From<legacy_crypto::LegacyCryptoError> for SecureAdapterError {
    fn from(value: legacy_crypto::LegacyCryptoError) -> Self {
        SecureAdapterError::Legacy(value)
    }
}

/// Adaptador que permite a `SecureData` comunicarse con la API legacy.
///
/// - Encapsula completamente el uso de `unsafe`.
/// - Valida precondiciones antes de llamar a la librería vieja.
#[derive(Debug, Default)]
pub struct LegacyAdapter;

impl LegacyAdapter {
    /// Crea un nuevo `LegacyAdapter`.
    pub fn new() -> Self {
        Self
    }

    /// Encripta los datos usando la API legacy.
    ///
    /// # Seguridad
    /// - Rechaza buffers vacíos.
    /// - Convierte internamente `Vec<u8>` a puntero crudo + longitud.
    /// - El `unsafe` está completamente encapsulado aquí.
    pub fn encrypt_with_legacy(
        &self,
        secure_node: &SecureData,
    ) -> Result<Vec<u8>, SecureAdapterError> {
        if secure_node.is_empty() {
            return Err(SecureAdapterError::EmptyInput);
        }

        let (ptr, len) = secure_node.as_ptr_len();

        // SAFETY:
        // - `ptr` proviene de un Vec vivo dentro de `SecureData`.
        // - `len` coincide con el tamaño real del buffer.
        let encrypted = unsafe { legacy_crypto::c_api_encrypt(ptr, len) }?;

        Ok(encrypted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_rejects_empty_input() {
        let adapter = LegacyAdapter::new();
        let secure = SecureData::new(vec![], SecurityLevel::Low, "empty");
        let result = adapter.encrypt_with_legacy(&secure);
        assert!(matches!(result, Err(SecureAdapterError::EmptyInput)));
    }
}
