//! Simulación de almacenamiento "frío": donde la clave permanece cifrada.

#[derive(Debug, Clone)]
pub struct ColdStore {
    encrypted_key: Vec<u8>,
}

impl ColdStore {
    pub fn new(encrypted_key: &[u8]) -> Self {
        Self {
            encrypted_key: encrypted_key.to_vec(),
        }
    }

    pub fn read_encrypted_key(&self) -> Vec<u8> {
        // En un sistema real, aquí leerías desde disco, HSM, KMS, etc.
        self.encrypted_key.clone()
    }
}
