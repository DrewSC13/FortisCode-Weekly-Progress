use std::sync::Arc;

/// Estado Intrínseco (Compartido).
/// IssuerData con CA name, country, algorithm.
#[derive(Debug, PartialEq, Eq)]
pub struct IssuerData {
    pub ca_name: String,
    pub country: String,
    pub algorithm: String,
}

impl IssuerData {
    pub fn new(ca_name: &str, country: &str, algorithm: &str) -> Self {
        Self {
            ca_name: ca_name.to_string(),
            country: country.to_string(),
            algorithm: algorithm.to_string(),
        }
    }

    pub fn ca_name(&self) -> &str {
        &self.ca_name
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }
}

/// Estado Extrínseco (Único).
/// IndividualCertificate contiene `Arc<IssuerData>` + campos únicos.
#[derive(Debug)]
pub struct IndividualCertificate {
    issuer: Arc<IssuerData>,
    serial_number: u64,
    public_key: Vec<u8>,
    subject: String,
}

impl IndividualCertificate {
    pub fn new(
        issuer: Arc<IssuerData>,
        serial_number: u64,
        public_key: Vec<u8>,
        subject: String,
    ) -> Self {
        Self {
            issuer,
            serial_number,
            public_key,
            subject,
        }
    }

    pub fn issuer(&self) -> &Arc<IssuerData> {
        &self.issuer
    }

    pub fn serial_number(&self) -> u64 {
        self.serial_number
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}
