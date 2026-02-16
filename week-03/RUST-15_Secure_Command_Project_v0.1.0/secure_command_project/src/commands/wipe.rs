//! Comando concreto: WipeSensitiveData (sobrescribe un buffer con ceros).

use crate::{SecurityCommand, SystemState};

#[derive(Debug)]
pub struct WipeSensitiveDataCommand {
    wiped: bool,
}

impl WipeSensitiveDataCommand {
    pub fn new() -> Self {
        Self { wiped: false }
    }
}

impl SecurityCommand for WipeSensitiveDataCommand {
    fn execute(&mut self, system: &mut SystemState) -> Result<(), String> {
        let buf = system.audit_buffer_mut();
        if buf.is_empty() {
            return Err("audit buffer is empty".to_string());
        }

        for b in buf.iter_mut() {
            *b = 0u8;
        }

        self.wiped = true;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "WipeSensitiveDataCommand"
    }

    fn undo(&mut self, _system: &mut SystemState) -> Result<(), String> {
        // Wipe no es reversible.
        if self.wiped {
            return Err("wipe is not reversible".to_string());
        }
        Err("nothing to undo".to_string())
    }
}

impl Default for WipeSensitiveDataCommand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SystemState;

    #[test]
    fn wipe_sensitive_data_zeros_out_buffer() {
        let mut state = SystemState::new(vec![1, 2, 3], vec![10, 20, 30, 40]);
        let mut cmd = WipeSensitiveDataCommand::new();

        cmd.execute(&mut state).unwrap();

        assert_eq!(state.audit_buffer(), &[0, 0, 0, 0]);
    }
}
