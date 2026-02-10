use crate::DecryptionService;

/// Servicio real: RealVault.
/// Realiza el "descifrado" en memoria.
///
/// Implementacion (simulada y deterministica):
/// - XOR con 0xAA sobre cada byte
/// - convierte a String(UTF-8 lossy) para cumplir la firma requerida
#[derive(Debug, Default, Clone, Copy)]
pub struct RealVault;

impl RealVault {
    pub fn new() -> Self {
        Self
    }
}

impl DecryptionService for RealVault {
    fn decrypt(&self, data: Vec<u8>) -> String {
        let plain: Vec<u8> = data.into_iter().map(|b| b ^ 0xAA).collect();
        String::from_utf8_lossy(&plain).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unitarias: Verificar que el REalVault descifra correctamente los datos.
    #[test]
    fn real_vault_decrypts_correctly() {
        let vault = RealVault::new();

        let plaintext = b"Hello".to_vec();
        let encrypted: Vec<u8> = plaintext.iter().map(|b| b ^ 0xAA).collect();

        let out = vault.decrypt(encrypted);
        assert_eq!(out, "Hello");
    }
}
