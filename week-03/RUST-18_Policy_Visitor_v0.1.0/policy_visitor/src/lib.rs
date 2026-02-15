//! RUST-18 - Policy Visitor(v0.1.0)
//!
//! Visitor para auditar politicas sobre elementos de seguridad sin modificar sus estructuras
//! Implementa doble despacho via `accept()`.
//!
//! Elementos:
//! - KeyStorage
//! - AccessLogs
//!
//! Visitantes:
//! - ExpirationChecker: detecta claves expiradas
//! - SizeAuditor: calcula el tamaño total (aproximado) de elementos.

#[path = "elements/keys.rs"]
pub mod keys;

#[path = "elements/logs.rs"]
pub mod logs;

#[path = "visitors/expiration.rs"]
pub mod expiration;

#[path = "visitors/size.rs"]
pub mod size;

pub use expiration::ExpirationChecker;
pub use keys::{KeyRecord, KeyStorage};
pub use logs::AccessLogs;
pub use size::SizeAuditor;

/// Trait Visitor: define operaciones para cada tipo concreto (doble despacho)
pub trait SecurityVisitor {
    fn visit_key_storage(&self, storage: &KeyStorage);
    fn visit_logs(&self, logs: &AccessLogs);
}

/// Trait Element: cada elemento acepta un visitante.
pub trait SecurityElement {
    fn accept(&self, visitor: &dyn SecurityVisitor);
}
