//! Comando concreto: Rotar claves (cambia la master key del sistema).

use crate::{SecurityCommand, SystemState};

#[derive(Debug)]
pub struct RotateKeysCommand {
    new_key: Vec<u8>,
    old_key: Option<Vec<u8>>,
}

impl RotateKeysCommand {
    pub fn new(new_key: Vec<u8>) -> Self {
        Self {
            new_key,
            old_key: None,
        }
    }
}

impl SecurityCommand for RotateKeysCommand {
    fn execute(&mut self, system: &mut SystemState) -> Result<(), String> {
        if self.new_key.is_empty() {
            return Err("new key cannot be empty".to_string());
        }

        self.old_key = Some(system.master_key().to_vec());
        system.set_master_key(self.new_key.clone());
        Ok(())
    }

    fn name(&self) -> &'static str {
        "RotateKeysCommand"
    }

    fn undo(&mut self, system: &mut SystemState) -> Result<(), String> {
        let old = self
            .old_key
            .take()
            .ok_or_else(|| "cannot undo: no previous key stored".to_string())?;

        system.set_master_key(old);
        Ok(())
    }
}
