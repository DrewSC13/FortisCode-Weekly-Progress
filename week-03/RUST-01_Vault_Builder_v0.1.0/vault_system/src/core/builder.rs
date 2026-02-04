use super::types::{CipherAlgorithm, EncryptionLevel, Vault};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_TIMEOUT_SECONDS: u64 = 300;

#[derive(Debug, Default)]
pub struct VaultBuilder {
    level: Option<EncryptionLevel>,
    algorithm: Option<CipherAlgorithm>,
    timeout_seconds: Option<u64>,
    secret_key: Option<Vec<u8>>,
}

impl VaultBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level(mut self, level: EncryptionLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn algorithm(mut self, algo: CipherAlgorithm) -> Self {
        self.algorithm = Some(algo);
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_seconds = Some(secs);
        self
    }

    pub fn key(mut self, data: Vec<u8>) -> Self {
        self.secret_key = Some(data);
        self
    }

    /// Construye el Vault final o retorna error si la configuracion no es segura
    ///
    /// Reglas:
    /// - timeout por defecto = 300 si no se especifica
    /// - key debe existir y no estar vacia
    /// - si level = High y algorithm = AES128 => Err
    pub fn build(self) -> Result<Vault, String> {
        let level = self
            .level
            .ok_or_else(|| "Falta definir ek nuvel de cifrado (EncryptionLevel)".to_string())?;

        let algorithm = self
            .algorithm
            .ok_or_else(|| "Falta definir el algoritmo (CipherAlgorithm).".to_string())?;

        let timeout_seconds = self.timeout_seconds.unwrap_or(DEFAULT_TIMEOUT_SECONDS);

        let secret_key = self
            .secret_key
            .ok_or_else(|| "Falta definir la clave secreta (secret_key).".to_string())?;

        if secret_key.is_empty() {
            return Err("La clave secreta (secret_key) no puede estar vacia.".to_string());
        }

        //Validacion principal pedida:
        if level == EncryptionLevel::High && algorithm == CipherAlgorithm::AES128 {
            return Err(
                "Configuracion insegura: nivel High no permite algoritmo AES128.".to_string(),
            );
        }

        let id = generate_id();

        Ok(Vault::new(
            id,
            level,
            algorithm,
            timeout_seconds,
            secret_key,
        ))
    }
}

fn generate_id() -> String {
    //ID simple sin crates extras: basado en epoch nanos
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);

    format!("vault-{}", nanos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{CipherAlgorithm, EncryptionLevel};

    #[test]
    fn build_success_creates_expected_vault() {
        let v = VaultBuilder::new()
            .level(EncryptionLevel::Medium)
            .algorithm(CipherAlgorithm::ChaCha20)
            .timeout(600)
            .key(vec![1, 2, 3, 4])
            .build()
            .unwrap();

        assert!(!v.get_id().is_empty());
        assert_eq!(v.get_level(), EncryptionLevel::Medium);
        assert_eq!(v.get_algorithm(), CipherAlgorithm::ChaCha20);
        assert_eq!(v.get_timeout_seconds(), 600);
        assert_eq!(v.secret_key_len(), 4);
    }

    #[test]
    fn security_validation_high_with_aes128_returns_err() {
        let res = VaultBuilder::new()
            .level(EncryptionLevel::High)
            .algorithm(CipherAlgorithm::AES128)
            .timeout(300)
            .key(vec![9, 9, 9])
            .build();

        assert!(res.is_err());
    }

    #[test]
    fn default_timeout_is_300_when_not_set() {
        let v = VaultBuilder::new()
            .level(EncryptionLevel::Low)
            .algorithm(CipherAlgorithm::AES128)
            .key(vec![7, 7, 7, 7])
            .build()
            .unwrap();

        assert_eq!(v.get_timeout_seconds(), 300);
    }
}
