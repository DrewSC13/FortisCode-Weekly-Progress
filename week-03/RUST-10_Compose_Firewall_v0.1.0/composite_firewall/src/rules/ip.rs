use crate::{Packet, SecurityRule};

/// Regla hoja: bloqueo por IP.
///
/// - Si `packet.src_ip` coincide con la IP bloqueada -> RECHAZA (false)
/// - Si no coincide -> PERMITE (true)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpRule {
    blocked_ip: String,
}

impl IpRule {
    pub fn new(blocked_ip: impl Into<String>) -> Self {
        Self {
            blocked_ip: blocked_ip.into(),
        }
    }

    pub fn blocked_ip(&self) -> &str {
        &self.blocked_ip
    }
}

impl SecurityRule for IpRule {
    fn check(&self, packet: &Packet) -> bool {
        packet.src_ip != self.blocked_ip
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unitarias: Verificar que una IpRule bloquea una IP específica.
    #[test]
    fn ip_rule_blocks_specific_ip() {
        let rule = IpRule::new("10.0.0.8");

        let blocked = Packet::new("10.0.0.8", 443);
        let allowed = Packet::new("10.0.0.9", 443);

        assert!(!rule.check(&blocked));
        assert!(rule.check(&allowed));
    }
}
