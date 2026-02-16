//! RUST-15 — Secure Command
//!
//! Encapsula operaciones sensibles como comandos para:
//! - Ejecutarlas de forma controlada
//! - Encolarlas
//! - Auditar qué se ejecutó
//! - Permitir undo en comandos que lo soporten

pub mod system_state;

pub mod commands {
    pub mod rotate;
    pub mod wipe;
}

use std::fmt;

pub use commands::rotate::RotateKeysCommand;
pub use commands::wipe::WipeSensitiveDataCommand;
pub use system_state::SystemState;

/// Trait Command incluye undo().
pub trait SecurityCommand: fmt::Debug {
    fn execute(&mut self, system: &mut SystemState) -> Result<(), String>;
    fn name(&self) -> &'static str;

    fn undo(&mut self, _system: &mut SystemState) -> Result<(), String> {
        Err("undo not supported".to_string())
    }
}

/// Invoker / Gestor: encola comandos, los ejecuta y guarda historial.
#[derive(Debug)]
pub struct CommandController {
    queue: Vec<Box<dyn SecurityCommand>>,
    history: Vec<String>,
}

impl CommandController {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, cmd: Box<dyn SecurityCommand>) {
        self.queue.push(cmd);
    }

    /// Ejecuta todos los comandos en orden, registrando el historial.
    pub fn execute_all(&mut self, system: &mut SystemState) -> Result<(), String> {
        for cmd in self.queue.iter_mut() {
            cmd.execute(system)?;
            self.history.push(cmd.name().to_string());
        }
        Ok(())
    }

    /// Historial exacto de operaciones ejecutadas.
    pub fn history(&self) -> &[String] {
        &self.history
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}

impl Default for CommandController {
    fn default() -> Self {
        Self::new()
    }
}
