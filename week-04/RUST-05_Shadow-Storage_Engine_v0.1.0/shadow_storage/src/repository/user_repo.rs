use crate::crypto::aes_engine;
use crate::crypto::aes_engine::{CryptoError, KEY_LEN};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use once_cell::sync::OnceCell;
use std::fs;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,       // sensible
    pub national_id: String, // sensible
}

#[derive(Debug, Clone)]
pub struct EncryptedUserRow {
    pub id: i64,
    pub username: String,
    pub email_blob: Vec<u8>,       // BYTEA
    pub national_id_blob: Vec<u8>, // BYTEA
}

/// Cache de key 32 bytes (estilo Docker Secret / volumen seguro)
static KEY_CACHE: OnceCell<[u8; KEY_LEN]> = OnceCell::new();

#[derive(Debug)]
pub enum KeyLoadError {
    MissingEnv,
    Io(std::io::Error),
    InvalidFormat(String),
}

impl std::fmt::Display for KeyLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyLoadError::MissingEnv => {
                write!(f, "missing SHADOW_KEY_FILE/SHADOW_KEY_B64/SHADOW_KEY_HEX")
            }
            KeyLoadError::Io(e) => write!(f, "key read io error: {e}"),
            KeyLoadError::InvalidFormat(s) => write!(f, "invalid key format: {s}"),
        }
    }
}
impl std::error::Error for KeyLoadError {}

/// Carga key 32 bytes desde:
/// 1) SHADOW_KEY_FILE (Docker Secret, recomendado: /run/secrets/shadow_key)
/// 2) SHADOW_KEY_B64  (base64 de 32 bytes)
/// 3) SHADOW_KEY_HEX  (hex de 32 bytes => 64 chars)
pub fn load_key_32() -> Result<&'static [u8; KEY_LEN], KeyLoadError> {
    KEY_CACHE.get_or_try_init(|| {
        if let Ok(path) = std::env::var("SHADOW_KEY_FILE") {
            let raw = fs::read(path).map_err(KeyLoadError::Io)?;
            return parse_key_bytes(&raw);
        }

        if let Ok(b64) = std::env::var("SHADOW_KEY_B64") {
            let bytes = STANDARD
                .decode(b64.trim())
                .map_err(|e| KeyLoadError::InvalidFormat(format!("base64 decode: {e}")))?;
            return parse_key_bytes(&bytes);
        }

        if let Ok(hexs) = std::env::var("SHADOW_KEY_HEX") {
            let bytes = hex::decode(hexs.trim())
                .map_err(|e| KeyLoadError::InvalidFormat(format!("hex decode: {e}")))?;
            return parse_key_bytes(&bytes);
        }

        Err(KeyLoadError::MissingEnv)
    })
}

fn parse_key_bytes(raw: &[u8]) -> Result<[u8; KEY_LEN], KeyLoadError> {
    if raw.len() == KEY_LEN {
        let mut k = [0u8; KEY_LEN];
        k.copy_from_slice(raw);
        return Ok(k);
    }

    // Secret puede venir con newline/spaces
    let trimmed = raw
        .iter()
        .copied()
        .filter(|b| *b != b'\n' && *b != b'\r' && *b != b' ' && *b != b'\t')
        .collect::<Vec<u8>>();

    if trimmed.len() == KEY_LEN {
        let mut k = [0u8; KEY_LEN];
        k.copy_from_slice(&trimmed);
        return Ok(k);
    }

    Err(KeyLoadError::InvalidFormat(format!(
        "expected 32 raw bytes (or 32 trimmed), got {} bytes",
        raw.len()
    )))
}

/// User -> Row cifrada (lo que se persiste como BYTEA)
pub fn to_encrypted_row(u: &User) -> Result<EncryptedUserRow, CryptoError> {
    let key = load_key_32().map_err(|_| CryptoError::InvalidKeyLen { got: 0 })?;
    let email_blob = aes_engine::encrypt(&u.email, key)?;
    let national_id_blob = aes_engine::encrypt(&u.national_id, key)?;

    Ok(EncryptedUserRow {
        id: u.id,
        username: u.username.clone(),
        email_blob,
        national_id_blob,
    })
}

/// Row cifrada -> User (lo que el servicio devuelve)
pub fn from_encrypted_row(r: &EncryptedUserRow) -> Result<User, CryptoError> {
    let key = load_key_32().map_err(|_| CryptoError::InvalidKeyLen { got: 0 })?;
    let email = aes_engine::decrypt(&r.email_blob, key)?;
    let national_id = aes_engine::decrypt(&r.national_id_blob, key)?;

    Ok(User {
        id: r.id,
        username: r.username.clone(),
        email,
        national_id,
    })
}

/// SQL de referencia: campos sensibles como BYTEA (ilegibles en SELECT)
pub const POSTGRES_SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS users (
  id BIGSERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email_enc BYTEA NOT NULL,
  national_id_enc BYTEA NOT NULL
);
"#;
