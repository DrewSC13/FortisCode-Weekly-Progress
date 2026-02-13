# 🟦 RUST-14 — Signature Observer

# RUST-14 — Signature Observer

## 📌 Descripción
Implementación del patrón Observer para notificar a múltiples módulos de seguridad cuando se detecta una firma digital inválida, permitiendo una comunicación uno-a-muchos desacoplada.

## 🧱 Funcionalidades
- Trait `SecurityObserver` con `update(&self, event_details: &str)`
- Subject `SignatureValidator` con:
  - `add_observer`
  - `validate_signature(is_valid: bool)`
- Observers:
  - `AuditLogger` → registra eventos en memoria (`Vec<String>`)
  - `AlertSystem` → activa una “Alerta Roja”
  - `IntrusionPreventer` → agrega la IP a una lista negra

## 🔐 Seguridad y Diseño
- Patrón Observer desacoplado: el validador no conoce la lógica interna de cada módulo.
- Interior mutability (`RefCell` / `Cell`) para permitir actualización con `&self` en `update`.
- Permite añadir nuevos observers sin modificar el Subject.

🧩 Cómo utilizarlo
use signature_observer::{SignatureValidator, AuditLogger, AlertSystem, IntrusionPreventer};
use std::sync::Arc;

let mut validator = SignatureValidator::new("INVALID_SIGNATURE ip=10.0.0.7");

let logger = Arc::new(AuditLogger::new());
let alerts = Arc::new(AlertSystem::new());
let ips = Arc::new(IntrusionPreventer::new());

validator.add_observer(Box::new(Arc::clone(&logger)));
validator.add_observer(Box::new(Arc::clone(&alerts)));
validator.add_observer(Box::new(Arc::clone(&ips)));

validator.validate_signature(false);

assert_eq!(logger.logs().len(), 1);
assert!(alerts.is_red_alert());
assert_eq!(ips.blacklist(), vec!["10.0.0.7".to_string()]);

⚙ Cómo funciona

Cuando validate_signature(false) es ejecutado, SignatureValidator recorre su lista de observers registrados y llama a update() en cada uno, permitiendo que:

Auditoría registre el evento,

Alertas active una bandera crítica,

IPS bloquee la IP relacionada al incidente.

Estado: Sprint 03 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC


## ▶ Ejecución
```bash
cargo fmt
cargo clippy
cargo audit
cargo doc
cargo test
