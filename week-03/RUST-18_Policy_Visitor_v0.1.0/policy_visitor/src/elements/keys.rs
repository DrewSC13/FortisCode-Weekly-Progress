use crate::{SecurityElement, SecurityVisitor};

/// Registro individual de clave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyRecord {
    pub id: String,
    pub expires_year: u32,
}

impl KeyRecord {
    pub fn new(id: impl Into<String>, expires_year: u32) -> Self {
        Self {
            id: id.into(),
            expires_year,
        }
    }
}

/// Almacén de claves, contiene múltiples registros.
#[derive(Debug, Default)]
pub struct KeyStorage {
    keys: Vec<KeyRecord>,
}

impl KeyStorage {
    pub fn new(keys: Vec<KeyRecord>) -> Self {
        Self { keys }
    }

    pub fn add(&mut self, key: KeyRecord) {
        self.keys.push(key);
    }

    pub fn keys(&self) -> &[KeyRecord] {
        &self.keys
    }

    /// Tamaño aproximado del storage (simulacion).
    pub fn size_bytes_estimate(&self) -> usize {
        // Aproximacion: suma de IDs + 4 bytes por año.
        self.keys.iter().map(|k| k.id.len() + 4).sum::<usize>()
    }
}

impl SecurityElement for KeyStorage {
    fn accept(&self, visitor: &dyn SecurityVisitor) {
        visitor.visit_key_storage(self);
    }
}
