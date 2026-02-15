use crate::{SecurityElement, SecurityVisitor};

/// Logs de acceso simulados.
#[derive(Debug, Default)]
pub struct AccessLogs {
    entries: Vec<String>,
}

impl AccessLogs {
    pub fn new(entries: Vec<String>) -> Self {
        Self { entries }
    }

    pub fn add(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Tamaño aproximado en bytes (simulacion).
    pub fn size_bytes_estimate(&self) -> usize {
        self.entries.iter().map(|e| e.len()).sum()
    }
}

impl SecurityElement for AccessLogs {
    fn accept(&self, visitor: &dyn SecurityVisitor) {
        visitor.visit_logs(self);
    }
}
