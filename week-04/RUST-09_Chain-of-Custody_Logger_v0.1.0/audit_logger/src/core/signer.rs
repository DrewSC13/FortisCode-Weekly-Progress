use base64::{Engine as _, engine::general_purpose::STANDARD};
use ed25519_dalek::Signer; // necesario para .sign()
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("invalid private seed: expected 32 bytes as hex (64 chars)")]
    InvalidSeed,
    #[error("signature decode/parse error")]
    BadSignature,
}

pub struct SignerWrap {
    signing: SigningKey,
    verifying: VerifyingKey,
}

impl SignerWrap {
    pub fn from_hex_seed(seed_hex: &str) -> Result<Self, SignerError> {
        let seed = hex::decode(seed_hex).map_err(|_| SignerError::InvalidSeed)?;
        if seed.len() != 32 {
            return Err(SignerError::InvalidSeed);
        }
        let signing = SigningKey::from_bytes(seed.as_slice().try_into().unwrap());
        let verifying = signing.verifying_key();
        Ok(Self { signing, verifying })
    }

    pub fn sign_current_hash_b64(&self, current_hash_hex: &str) -> String {
        let msg = hex::decode(current_hash_hex).expect("current_hash hex must decode");
        let sig: Signature = self.signing.sign(&msg);
        STANDARD.encode(sig.to_bytes())
    }

    pub fn verify_current_hash_b64(
        &self,
        current_hash_hex: &str,
        signature_b64: &str,
    ) -> Result<bool, SignerError> {
        let msg = hex::decode(current_hash_hex).map_err(|_| SignerError::BadSignature)?;
        let sig_bytes = STANDARD
            .decode(signature_b64)
            .map_err(|_| SignerError::BadSignature)?;
        let sig = Signature::from_slice(&sig_bytes).map_err(|_| SignerError::BadSignature)?;
        Ok(self.verifying.verify_strict(&msg, &sig).is_ok())
    }
}
