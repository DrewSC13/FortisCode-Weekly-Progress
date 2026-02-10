//! Proxy que intercepta llamadas a descifrado para:
//! - registrar intentos (auditoria de timstamp)
//! - aplicar control (bloquear payload sospechoso grande)
//! - delegar al servicio real (RealVault)

/// Trait de Servicio:define decrypt(&self, data: Vec<u/// Trait de Servicio: define decrypt(&self, data: `Vec<u8>`) -> String.8>) -> String.
pub trait DecryptionService {
    fn decrypt(&self, data: Vec<u8>) -> String;
}

#[path = "vault.rs"]
pub mod vault;

#[path = "proxy.rs"]
pub mod proxy;

pub use proxy::AuditProxy;
pub use vault::RealVault;
