use crate::{Packet, SecurityRule};

/// Componente compuesto (composite):
///
/// Requisito:
/// - Vec<Box<dyn SecurityRule>>
/// - check() retorna true SOLO si TODAS las reglas internas permiten el paquete.
/// - Debe permitir contener otros RuleGroup (jerarquia)
#[derive(Default)]
pub struct RuleGroup {
    rules: Vec<Box<dyn SecurityRule>>,
}

impl RuleGroup {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule<R: SecurityRule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl SecurityRule for RuleGroup {
    fn check(&self, packet: &Packet) -> bool {
        // true solo si todas permiten
        self.rules.iter().all(|r| r.check(packet))
    }
}
