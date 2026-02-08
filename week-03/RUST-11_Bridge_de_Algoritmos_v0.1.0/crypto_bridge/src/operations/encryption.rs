use crate::CryptoBackend;

/// Lado de la Abstracción: Cifrado de archivos.
/// Contiene `backend: Box<dyn CryptoBackend>`.
pub struct FileEncryption {
    backend: Box<dyn CryptoBackend>,
}

impl FileEncryption {
    pub fn new(backend: Box<dyn CryptoBackend>) -> Self {
        Self { backend }
    }

    /// Permite intercambiar backend en tiempo de ejecución.
    pub fn set_backend(&mut self, backend: Box<dyn CryptoBackend>) {
        self.backend = backend;
    }

    /// Operación de alto nivel: cifrar “archivo”.
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut payload = b"FILE:".to_vec();
        payload.extend_from_slice(data);
        self.backend.raw_encrypt(&payload)
    }

    /// Operación de alto nivel: descifrar “archivo”.
    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let decoded = self.backend.raw_decrypt(data);
        if decoded.starts_with(b"FILE:") {
            decoded[5..].to_vec()
        } else {
            decoded
        }
    }
}
