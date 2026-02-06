//! RUST-07 — Encryption Decorator (v0.1.0)
//!
//! Patrón Decorator usando Traits para apilar capas de procesamiento.
//!
//! Composición requerida:
//! `let logger = Base64Decorator(AesDecorator(PlainTextWriter));`

pub mod writers;

/// Trait Base: SecureWriter con write(&self, data: &str) -> String.
pub trait SecureWriter {
    fn write(&self, data: &str) -> String;
}

#[path = "decorators/aes.rs"]
pub mod aes;

#[path = "decorators/base64.rs"]
pub mod base64;

pub use aes::AesDecorator;
pub use base64::Base64Decorator;
pub use writers::PlainTextWriter;
