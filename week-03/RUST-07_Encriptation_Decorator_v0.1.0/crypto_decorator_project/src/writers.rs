use crate::SecureWriter;

/// Componente Base: devuelve el string tal cual.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlainTextWriter;

impl SecureWriter for PlainTextWriter {
    fn write(&self, data: &str) -> String {
        data.to_string()
    }
}
