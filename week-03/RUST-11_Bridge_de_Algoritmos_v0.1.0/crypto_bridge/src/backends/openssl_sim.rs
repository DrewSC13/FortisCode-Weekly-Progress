use crate::CryptoBackend;

/// Implementación técnica: OpenSSLBackend (simulado).
/// Implementar CryptoBackend.
///
/// Implementación reversible (simulada):
/// XOR con una clave fija distinta al backend nativo.
#[derive(Debug, Default, Clone, Copy)]
pub struct OpenSSLBackend;

const KEY: u8 = 0xA7;

impl CryptoBackend for OpenSSLBackend {
    fn raw_encrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ KEY).collect()
    }

    fn raw_decrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ KEY).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openssl_backend_encrypt_decrypt_roundtrip() {
        let backend = OpenSSLBackend;
        let msg = b"SIG:mensaje-firma";

        let enc = backend.raw_encrypt(msg);
        assert_ne!(enc, msg);

        let dec = backend.raw_decrypt(&enc);
        assert_eq!(dec, msg);
    }
}
