# 🟦 RUST-10 — Composite Firewall

# RUST-10 — Composite Firewall

## 📌 Descripción
Implementación del patrón Composite para un firewall, donde una regla de seguridad puede ser una regla simple (hoja) o un grupo de reglas (composición), permitiendo evaluar estructuras jerárquicas de reglas como una sola unidad.

## 🧱 Funcionalidades
- Definición del trait `SecurityRule`
- Reglas hoja:
  - `IpRule` (bloqueo por IP)
  - `PortRule` (bloqueo por puerto)
- Regla compuesta `RuleGroup` con soporte de jerarquía
- Evaluación de reglas tipo AND (todas deben permitir)

## 🔐 Seguridad de Memoria
- Uso de `Box<dyn SecurityRule>` para polimorfismo seguro
- Encapsulación de reglas sin uso de `unsafe`
- Evaluación inmutable de paquetes (`&Packet`)

## ▶ Ejecución
```bash
cargo fmt
cargo clippy
cargo audit
cargo test

🧩 Cómo utilizarlo
use composite_firewall::{Packet, IpRule, PortRule, RuleGroup, SecurityRule};

let mut ip_group = RuleGroup::new();
ip_group.add_rule(IpRule::new("10.0.0.8"));

let mut firewall = RuleGroup::new();
firewall.add_rule(PortRule::new(22));
firewall.add_rule(ip_group);

let packet = Packet::new("1.2.3.4", 80);
assert!(firewall.check(&packet));

⚙ Cómo funciona
Cada regla implementa el trait SecurityRule

RuleGroup evalúa todas sus reglas internas

Si cualquier regla falla, el paquete es rechazado

RuleGroup puede contener otros RuleGroup, formando un árbol de reglas

Estado: Sprint 03 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC
