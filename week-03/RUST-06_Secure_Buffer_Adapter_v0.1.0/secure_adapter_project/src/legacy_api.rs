//! Simulación de librería criptográfica legacy (estilo C).
//! Requisito: módulo `legacy_crypto` con función:
//! `c_api_encrypt(data: *const u8, len: usize)`

use core::fmt;

pub mod legacy_crypto {
    use super::*;

    /// Errores simulados de la API legacy.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum LegacyCryptoError {
        NullPointer,
        InvalidLength,
    }

    impl fmt::Display for LegacyCryptoError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LegacyCryptoError::NullPointer => write!(f, "legacy api received null pointer"),
                LegacyCryptoError::InvalidLength => write!(f, "legacy api received invalid length"),
            }
        }
    }

    impl std::error::Error for LegacyCryptoError {}

    /// API legacy estilo C: recibe puntero crudo + longitud.
    ///
    /// “Cifrado” simulado: XOR con 0xAA para poder verificar determinísticamente.
    ///
    /// # Safety
    /// - `data` debe ser válido para lectura de `len` bytes.
    /// - `len` debe ser consistente con el buffer real.
    pub unsafe fn c_api_encrypt(data: *const u8, len: usize) -> Result<Vec<u8>, LegacyCryptoError> {
        if data.is_null() {
            return Err(LegacyCryptoError::NullPointer);
        }
        if len == 0 {
            return Err(LegacyCryptoError::InvalidLength);
        }

        // SAFETY:
        // - Ya validamos que `data` no es null.
        // - El contrato de la función (caller) garantiza que `data` apunta a `len` bytes válidos.
        let input = unsafe { core::slice::from_raw_parts(data, len) };

        let mut out = Vec::with_capacity(len);
        for &b in input {
            out.push(b ^ 0xAA);
        }
        Ok(out)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn rejects_null_pointer() {
            let r = unsafe { c_api_encrypt(core::ptr::null(), 5) };
            assert_eq!(r, Err(LegacyCryptoError::NullPointer));
        }

        #[test]
        fn rejects_zero_length() {
            let data = [1u8, 2, 3];
            let r = unsafe { c_api_encrypt(data.as_ptr(), 0) };
            assert_eq!(r, Err(LegacyCryptoError::InvalidLength));
        }

        #[test]
        fn encrypts_with_xor() {
            let data = [0x00u8, 0xFFu8];
            let r = unsafe { c_api_encrypt(data.as_ptr(), data.len()) }.unwrap();
            assert_eq!(r, vec![0xAAu8, 0x55u8]);
        }
    }
}
