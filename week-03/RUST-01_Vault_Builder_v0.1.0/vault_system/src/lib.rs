//! Vault Systen
//! 
//! Builder para configurar una "Boveda Digital" (vault) con validaciones de seguridad

mod core;

pub use core::types::{CipherAlgorithm, EncryptionLevel, Vault};
pub use core::builder::VaultBuilder;