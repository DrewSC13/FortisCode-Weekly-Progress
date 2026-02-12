use crate::models::IndividualCertificate;

/// Simulación simple de almacenamiento en memoria.
#[derive(Default)]
pub struct CertificateStore {
    certs: Vec<IndividualCertificate>,
}

impl CertificateStore {
    pub fn new() -> Self {
        Self { certs: Vec::new() }
    }

    pub fn add(&mut self, cert: IndividualCertificate) {
        self.certs.push(cert);
    }

    pub fn len(&self) -> usize {
        self.certs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.certs.is_empty()
    }

    pub fn all(&self) -> &[IndividualCertificate] {
        &self.certs
    }
}
