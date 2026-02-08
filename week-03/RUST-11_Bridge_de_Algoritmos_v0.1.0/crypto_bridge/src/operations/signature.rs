use crate::CryptoBackend;

/// Lado de la Abstracción: Firma Digital.
/// Contiene `backend: Box<dyn CryptoBackend>`.
pub struct DigitalSignature {
    backend: Box<dyn CryptoBackend>,
}

impl DigitalSignature {
    pub fn new(backend: Box<dyn CryptoBackend>) -> Self {
        Self { backend }
    }

    /// Permite intercambiar el backend en tiempo de ejecución.
    pub fn set_backend(&mut self, backend: Box<dyn CryptoBackend>) {
        self.backend = backend;
    }

    /// Operación de alto nivel: firmar.
    /// - No sabe qué backend hay debajo.
    /// - Delegación al backend mediante raw_encrypt.
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let mut payload = b"SIG:".to_vec();
        payload.extend_from_slice(message);
        self.backend.raw_encrypt(&payload)
    }

    /// Operación de alto nivel: verificar.
    /// - Invierte usando raw_decrypt y valida prefijo + mensaje.
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        let decoded = self.backend.raw_decrypt(signature);

        if !decoded.starts_with(b"SIG:") {
            return false;
        }
        &decoded[4..] == message
    }
}
