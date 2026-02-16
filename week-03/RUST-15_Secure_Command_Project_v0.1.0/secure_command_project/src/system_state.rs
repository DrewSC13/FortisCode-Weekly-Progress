//! Estado del sistema que los comandos modifican.

#[derive(Debug, Clone)]
pub struct SystemState {
    master_key: Vec<u8>,
    audit_buffer: Vec<u8>,
}

impl SystemState {
    pub fn new(master_key: Vec<u8>, audit_buffer: Vec<u8>) -> Self {
        Self {
            master_key,
            audit_buffer,
        }
    }

    pub fn master_key(&self) -> &[u8] {
        &self.master_key
    }

    pub fn set_master_key(&mut self, new_key: Vec<u8>) {
        self.master_key = new_key;
    }

    pub fn audit_buffer(&self) -> &[u8] {
        &self.audit_buffer
    }

    pub fn audit_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.audit_buffer
    }
}
