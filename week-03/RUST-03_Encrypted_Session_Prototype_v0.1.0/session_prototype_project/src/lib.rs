//! Encrypted Session Prototype
//!
//! Implementacion del patron Prototype para clonar sesiones sin copiar tokens sensibles.

pub mod crypto_utils;
pub mod session;

/// Trait propio para el patron Protorype.
pub trait SessionPrototype {
    fn spawn_clone(&self) -> Self;
}

pub use session::Session;
