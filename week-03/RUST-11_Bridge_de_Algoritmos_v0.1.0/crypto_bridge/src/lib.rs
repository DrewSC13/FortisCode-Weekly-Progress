//! Bridge: separa la Abstracción (operaciones de alto nivel)
//! de la Implementación (backend/algoritmo), permitiendo intercambiar
//! backends sin afectar al cliente.

/// Lado de la Implementación: backend criptográfico.
/// Trait CryptoBackend con métodos raw_encrypt y raw_decrypt.
pub trait CryptoBackend {
    fn raw_encrypt(&self, data: &[u8]) -> Vec<u8>;
    fn raw_decrypt(&self, data: &[u8]) -> Vec<u8>;
}

#[path = "backends/openssl_sim.rs"]
pub mod openssl_sim;

#[path = "backends/rust_crypto.rs"]
pub mod rust_crypto;

#[path = "operations/signature.rs"]
pub mod signature;

#[path = "operations/encryption.rs"]
pub mod encryption;

pub use encryption::FileEncryption;
pub use openssl_sim::OpenSSLBackend;
pub use rust_crypto::NativeRustBackend;
pub use signature::DigitalSignature;
