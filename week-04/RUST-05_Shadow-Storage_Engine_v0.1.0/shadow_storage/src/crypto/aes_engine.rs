use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use rand_core::RngCore;
use std::fmt;

pub const NONCE_LEN: usize = 12; // 96-bit
pub const TAG_LEN: usize = 16; // 128-bit tag
pub const KEY_LEN: usize = 32; // 256-bit

#[derive(Debug, Clone)]
pub enum CryptoError {
    InvalidKeyLen { got: usize },
    InvalidBlob { len: usize },
    AuthFailed,
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidKeyLen { got } => {
                write!(f, "invalid key length: expected 32, got {got}")
            }
            CryptoError::InvalidBlob { len } => {
                write!(f, "invalid blob: len={len} (need >= 12 + 16)")
            }
            CryptoError::AuthFailed => write!(f, "authentication failed (tampered / tag mismatch)"),
            CryptoError::Utf8(e) => write!(f, "utf8 decode error: {e}"),
        }
    }
}

impl std::error::Error for CryptoError {}

/// Layout del blob (para DB):
/// [ 12 bytes NONCE | (CIPHERTEXT || 16 bytes TAG) ]
pub fn encrypt(plaintext: &str, key_32: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key_32.len() != KEY_LEN {
        return Err(CryptoError::InvalidKeyLen { got: key_32.len() });
    }

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key_32)
        .map_err(|_| CryptoError::InvalidKeyLen { got: key_32.len() })?;

    // ciphertext_with_tag = ciphertext || tag
    let ciphertext_with_tag = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| CryptoError::AuthFailed)?;

    let mut blob = Vec::with_capacity(NONCE_LEN + ciphertext_with_tag.len());
    blob.extend_from_slice(&nonce_bytes);
    blob.extend_from_slice(&ciphertext_with_tag);
    Ok(blob)
}

pub fn decrypt(blob: &[u8], key_32: &[u8]) -> Result<String, CryptoError> {
    if key_32.len() != KEY_LEN {
        return Err(CryptoError::InvalidKeyLen { got: key_32.len() });
    }
    if blob.len() < NONCE_LEN + TAG_LEN {
        return Err(CryptoError::InvalidBlob { len: blob.len() });
    }

    let (nonce_part, ct_part) = blob.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_part);

    let cipher = Aes256Gcm::new_from_slice(key_32)
        .map_err(|_| CryptoError::InvalidKeyLen { got: key_32.len() })?;

    let plaintext_bytes = cipher
        .decrypt(nonce, ct_part)
        .map_err(|_| CryptoError::AuthFailed)?;

    String::from_utf8(plaintext_bytes).map_err(CryptoError::Utf8)
}

/// Simula manipulación en DB/dump: voltea 1 bit del ciphertext (no del nonce).
pub fn flip_one_bit(mut blob: Vec<u8>) -> Vec<u8> {
    if blob.len() > NONCE_LEN {
        blob[NONCE_LEN] ^= 0b0000_0001;
    }
    blob
}
