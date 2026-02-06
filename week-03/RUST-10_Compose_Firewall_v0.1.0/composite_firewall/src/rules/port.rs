use crate::{Packet, SecurityRule};

/// Regla hoja: bloquea un puerto especifico
///
/// check() devuelve:
/// - false si el puerto coincide (rechazado)
/// - true si no coincide (permitido)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortRule {
    blocked_port: u16,
}

impl PortRule {
    pub fn new(blocked_port: u16) -> Self {
        Self { blocked_port }
    }

    pub fn blocked_port(&self) -> u16 {
        self.blocked_port
    }
}

impl SecurityRule for PortRule {
    fn check(&self, packet: &Packet) -> bool {
        packet.dst_port != self.blocked_port
    }
}
