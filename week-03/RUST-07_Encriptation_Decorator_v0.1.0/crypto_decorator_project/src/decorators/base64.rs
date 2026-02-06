use crate::SecureWriter;
use base64::{Engine as _, engine::general_purpose};

/// Decorador: recibe algo que implemente SecureWriter, aplica Base64 al resultado y lo devuelve.
///
/// Requisito de composición:
/// `Base64Decorator(AesDecorator(PlainTextWriter))`
#[derive(Debug, Clone, Copy)]
pub struct Base64Decorator<W: SecureWriter>(pub W);

impl<W: SecureWriter> SecureWriter for Base64Decorator<W> {
    fn write(&self, data: &str) -> String {
        let upstream = self.0.write(data);
        general_purpose::STANDARD.encode(upstream.as_bytes())
    }
}

impl<W: SecureWriter> Base64Decorator<W> {
    /// Proceso inverso (para integración): decodifica Base64 y retorna String.
    pub fn decode_to_string(b64: &str) -> Result<String, base64::DecodeError> {
        let bytes = general_purpose::STANDARD.decode(b64)?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writers::PlainTextWriter;

    /// Unitarias: Verificar que Base64Decorator por sí solo codifica correctamente texto plano.
    #[test]
    fn base64_decorator_encodes_plain_text_correctly() {
        let logger = Base64Decorator(PlainTextWriter);
        let out = logger.write("hola");
        assert_eq!(out, "aG9sYQ==");
    }
}
