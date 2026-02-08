use crate::CryptoBackend;

/// Impelemnacion tecnica: NativeRustBackend
/// implementando CryptoBackend
///
/// Implementacion reversible (simulada) para demostrar el Bridge:
/// XOR con una clave fija.
#[derive(Debug, Default, Clone, Copy)]
pub struct NativeRustBackend;

const KET: u8 = 0xAA;

impl CryptoBackend for NativeRustBackend {
    fn raw_encrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ KET).collect()
    }

    fn raw_decrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ KET).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CryptoBackend;

    /// Unitarias:
    /// Probar que el NAtiveRustBackend procesa bytes correctamente de forma aislada.
    #[test]
    fn native_backend_processes_bytes_correctly() {
        let backend = NativeRustBackend;

        let msg = b"Hello-bytes!";
        let encrypted = backend.raw_encrypt(msg);
        assert_ne!(encrypted, msg);

        let decrypted = backend.raw_decrypt(&encrypted);
        assert_eq!(decrypted, msg);
    }
}
