use crate::AuthStrategy;

/// Estrategia completa: BiometricAuth
/// Simula el escaneo de una firma digital de huella.
pub struct BiometricAuth {
    expected_signature: String,
}

impl BiometricAuth {
    pub fn new(expected_signature: &str) -> Self {
        Self {
            expected_signature: expected_signature.into(),
        }
    }
}

impl AuthStrategy for BiometricAuth {
    fn authenticate(&self, credentials: &str) -> bool {
        // Simulacion simple: comparacion exacta de la "firma"
        credentials == self.expected_signature
    }
}
