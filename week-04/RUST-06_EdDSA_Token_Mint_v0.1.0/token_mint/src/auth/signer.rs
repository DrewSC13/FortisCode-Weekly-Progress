use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize, // unix timestamp
    pub iss: String,
    pub sub: String,
}

pub struct Signer {
    issuer: String,
    encoding_key: EncodingKey,
    kid: String,
    ttl_minutes: i64,
}

impl Signer {
    /// Carga clave privada Ed25519 desde PEM (keys/private.pem) - SOLO servicio Auth.
    pub fn from_private_pem_file(
        private_pem_path: impl AsRef<Path>,
        kid: impl Into<String>,
        issuer: impl Into<String>,
        ttl_minutes: i64, // ej. 15
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let pem = fs::read(private_pem_path)?;
        let encoding_key = EncodingKey::from_ed_pem(&pem)?; 

        Ok(Self {
            issuer: issuer.into(),
            encoding_key,
            kid: kid.into(),
            ttl_minutes,
        })
    }

    /// Emite token JWT EdDSA:
    /// - exp corto (ej. 15 min)
    /// - iss emisor
    /// - sub id usuario
    /// - header.kid para rotación
    pub fn mint(&self, user_id: impl Into<String>) -> Result<String, jsonwebtoken::errors::Error> {
        let exp =
            (OffsetDateTime::now_utc() + Duration::minutes(self.ttl_minutes)).unix_timestamp();

        let claims = Claims {
            exp: exp as usize,
            iss: self.issuer.clone(),
            sub: user_id.into(),
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.kid = Some(self.kid.clone());

        jsonwebtoken::encode(&header, &claims, &self.encoding_key)
    }
}
