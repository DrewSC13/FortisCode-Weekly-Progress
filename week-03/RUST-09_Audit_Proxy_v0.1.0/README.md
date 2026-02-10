# 🟦 RUST-09 — Audit Proxy

# RUST-09 — Audit Proxy

## 📌 Descripción
Implementación del patrón Proxy para interceptar operaciones sensibles de descifrado, registrando cada intento con timestamp para auditoría y aplicando control de acceso basado en el tamaño del payload antes de delegar al servicio real.

## 🧱 Funcionalidades
- Trait `DecryptionService` con `decrypt(&self, data: Vec<u8>) -> String`
- Servicio real `RealVault` con descifrado en memoria (simulado)
- Proxy `AuditProxy`:
  - Registro de auditoría (`Vec<String>`) con timestamp
  - Bloqueo de peticiones con payload sospechosamente grande
  - Delegación al `RealVault` cuando la solicitud es válida

## 🔐 Seguridad de Memoria
- Sin uso de `unsafe`
- Uso de interior mutability (`RefCell`) para registrar auditoría manteniendo la firma `&self`
- Control preventivo de payloads grandes para evitar colapsos/límites de memoria

## ▶ Ejecución
```bash
cargo fmt
cargo clippy
cargo audit
cargo doc
cargo test

🧩 Cómo utilizarlo
use audit_proxy_project::{AuditProxy, DecryptionService, RealVault};

let vault = RealVault::new();
let proxy = AuditProxy::new(vault);

// Datos cifrados (simulado XOR 0xAA)
let encrypted: Vec<u8> = b"hola".iter().map(|b| b ^ 0xAA).collect();

let plaintext = proxy.decrypt(encrypted);
assert_eq!(plaintext, "hola");

⚙ Cómo funciona
AuditProxy implementa el mismo trait que el servicio real (DecryptionService).

Cada llamada a decrypt:

Registra "Intento de descifrado a las [timestamp]"

Valida el tamaño del payload (bloquea si es sospechoso)

Delega al RealVault si la solicitud es válida

El historial se mantiene dentro del proxy para auditoría y trazabilidad.

Estado: Sprint 03 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC
