use crate::SecureWriter;
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    InvalidHex,
    InvalidUtf8,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidHex => write!(f, "invalid hex input"),
            CryptoError::InvalidUtf8 => write!(f, "decrypted bytes are not valid UTF-8"),
        }
    }
}

impl std::error::Error for CryptoError {}

/// Decorador: recibe algo que implemente SecureWriter, cifra el resultado y lo devuelve.
///
/// Requisito de composición:
/// `AesDecorator(PlainTextWriter)`
///
/// Implementación:
/// - Cifrado simulado: XOR con clave fija.
/// - Devuelve ciphertext como HEX (String) para permitir Base64 encima.
#[derive(Debug, Clone, Copy)]
pub struct AesDecorator<W: SecureWriter>(pub W);

impl<W: SecureWriter> SecureWriter for AesDecorator<W> {
    fn write(&self, data: &str) -> String {
        let upstream = self.0.write(data);
        let cipher = xor_transform(upstream.as_bytes());
        hex_encode(&cipher)
    }
}

impl<W: SecureWriter> AesDecorator<W> {
    /// Proceso inverso: recibe HEX y devuelve el plaintext original.
    pub fn decrypt_hex(cipher_hex: &str) -> Result<String, CryptoError> {
        let cipher_bytes = hex_decode(cipher_hex)?;
        let plain_bytes = xor_transform(&cipher_bytes); // XOR es reversible
        String::from_utf8(plain_bytes).map_err(|_| CryptoError::InvalidUtf8)
    }
}

// ----------------- helpers internos -----------------

const KEY: &[u8] = b"RUST-07-KEY";

fn xor_transform(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    for (i, &b) in input.iter().enumerate() {
        out.push(b ^ KEY[i % KEY.len()]);
    }
    out
}

fn hex_encode(bytes: &[u8]) -> String {
    const LUT: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(LUT[(b >> 4) as usize] as char);
        s.push(LUT[(b & 0x0f) as usize] as char);
    }
    s
}

fn hex_decode(s: &str) -> Result<Vec<u8>, CryptoError> {
    let b = s.as_bytes();
    if !b.len().is_multiple_of(2) {
        return Err(CryptoError::InvalidHex);
    }

    fn val(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    }

    let mut out = Vec::with_capacity(b.len() / 2);
    let mut i = 0;
    while i < b.len() {
        let hi = val(b[i]).ok_or(CryptoError::InvalidHex)?;
        let lo = val(b[i + 1]).ok_or(CryptoError::InvalidHex)?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}
