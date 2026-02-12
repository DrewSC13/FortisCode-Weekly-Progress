//! RUST-12 — Flyweight de Certificados (v0.1.0)
//!
//! Objetivo: gestionar miles de certificados compartiendo estado intrínseco
//! (IssuerData) mediante Arc para ahorrar memoria.

use std::collections::HashMap;
use std::sync::Arc;

#[path = "models.rs"]
pub mod models;

#[path = "store.rs"]
pub mod store;

pub use models::{IndividualCertificate, IssuerData};

/// Flyweight Factory: reutiliza IssuerData compartido usando Arc.
///
/// `HashMap<String, Arc<IssuerData>>`.
/// - Si el emisor ya existe, devuelve la referencia existente.
/// - Si no, crea una nueva y la almacena.
pub struct CertificateFactory {
    issuers: HashMap<String, Arc<IssuerData>>,
}

impl CertificateFactory {
    pub fn new() -> Self {
        Self {
            issuers: HashMap::new(),
        }
    }

    /// Obtiene (o crea) un IssuerData compartido para un emisor.
    ///
    /// La clave se construye a partir de los campos del issuer para garantizar
    /// reutilización exacta cuando los datos son idénticos.
    pub fn get_issuer(&mut self, ca_name: &str, country: &str, algorithm: &str) -> Arc<IssuerData> {
        let key = make_issuer_key(ca_name, country, algorithm);

        if let Some(existing) = self.issuers.get(&key) {
            return Arc::clone(existing);
        }

        let created = Arc::new(IssuerData::new(ca_name, country, algorithm));
        self.issuers.insert(key, Arc::clone(&created));
        created
    }

    // Helper para crear un certificado infividual usando el issuer flyweight.
    pub fn create_certificate(
        &mut self,
        ca_name: &str,
        country: &str,
        algorithm: &str,
        serial_number: u64,
        public_key: Vec<u8>,
        subject: String,
    ) -> IndividualCertificate {
        let issuer = self.get_issuer(ca_name, country, algorithm);
        IndividualCertificate::new(issuer, serial_number, public_key, subject)
    }

    pub fn issuer_count(&self) -> usize {
        self.issuers.len()
    }
}

impl Default for CertificateFactory {
    fn default() -> Self {
        Self::new()
    }
}

fn make_issuer_key(ca_name: &str, country: &str, algorithm: &str) -> String {
    format!("{}|{}|{}", ca_name, country, algorithm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn factory_reuses_same_issuer_data_for_identical_requests() {
        let mut factory = CertificateFactory::new();

        let a = factory.get_issuer("FortisCA", "BO", "RSA-2048");
        let b = factory.get_issuer("FortisCA", "BO", "RSA-2048");

        assert!(Arc::ptr_eq(&a, &b));
        assert_eq!(factory.issuer_count(), 1);
    }
}
