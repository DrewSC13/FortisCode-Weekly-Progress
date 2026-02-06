//! Patrón Composite para reglas de seguridad:
//! - Regla hoja (IpRule / PortRule)
//! - Regla compuesta (RuleGroup) que puede contener sub-reglas (incluyendo otros grupos)

pub mod group;

#[path = "rules/ip.rs"]
pub mod ip;

#[path = "rules/port.rs"]
pub mod port;

/// Estructura Packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub src_ip: String,
    pub dst_port: u16,
}

impl Packet {
    pub fn new(src_ip: impl Into<String>, dst_port: u16) -> Self {
        Self {
            src_ip: src_ip.into(),
            dst_port,
        }
    }
}

/// Trait de Regla.
pub trait SecurityRule {
    /// true = permitido, false = rechazado
    fn check(&self, packet: &Packet) -> bool;
}

pub use group::RuleGroup;
pub use ip::IpRule;
pub use port::PortRule;
