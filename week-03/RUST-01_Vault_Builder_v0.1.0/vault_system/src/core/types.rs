#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherAlgorithm {
    AES128,
    AES256,
    ChaCha20,
}

/// Vault (Bóveda Digital).
///
/// Campos privados por diseño (encapsulación): un usuario externo NO debe acceder a ellos directamente.
#[derive(Debug)]
pub struct Vault {
    id: String,
    level: EncryptionLevel,
    algorithm: CipherAlgorithm,
    timeout_seconds: u64,
    secret_key: Vec<u8>,
}

impl Vault {
    /// Punto de entrada recomendado para construir un Vault.
    ///
    /// # Examples
    /// ```
    /// use vault_system::{Vault, EncryptionLevel, CipherAlgorithm};
    ///
    /// let v = Vault::builder()
    ///     .level(EncryptionLevel::High)
    ///     .algorithm(CipherAlgorithm::AES256)
    ///     .timeout(300)
    ///     .key(vec![1,2,3,4,5,6,7,8])
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(!v.get_id().is_empty());
    /// ```
    pub fn builder() -> crate::VaultBuilder {
        crate::VaultBuilder::new()
    }

    /// API pública: permite leer el id sin exponer campos privados.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_timeout_seconds(&self) -> u64 {
        self.timeout_seconds
    }

    pub fn get_level(&self) -> EncryptionLevel {
        self.level
    }

    pub fn get_algorithm(&self) -> CipherAlgorithm {
        self.algorithm
    }

    // Retorna la longitud de la clave secreta (sin exponerla).
    pub fn secret_key_len(&self) -> usize {
        self.secret_key.len()
    }
}

impl Vault {
    // Constructor interno para que solo el builder cree instancias.
    pub(crate) fn new(
        id: String,
        level: EncryptionLevel,
        algorithm: CipherAlgorithm,
        timeout_seconds: u64,
        secret_key: Vec<u8>,
    ) -> Self {
        Self {
            id,
            level,
            algorithm,
            timeout_seconds,
            secret_key,
        }
    }
}
