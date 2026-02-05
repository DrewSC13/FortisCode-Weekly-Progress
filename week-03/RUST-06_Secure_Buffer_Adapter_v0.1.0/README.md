# 🟦 RUST-06 — Secure Buffer Adapter

# RUST-06 — Secure Buffer Adapter

## 📌 Descripción
Adaptador de seguridad que permite integrar una estructura moderna (`SecureData`) con una librería criptográfica legacy basada en punteros crudos, encapsulando completamente el uso de `unsafe` y evitando la exposición directa de datos sensibles.

## 🧱 Funcionalidades
- Conversión segura de `Vec<u8>` a `*const u8 + len`
- Encapsulación total del código `unsafe`
- Limpieza de memoria sensible en `Drop`
- Manejo de errores provenientes de la API legacy

## 🔐 Seguridad de Memoria
- `SecureData` encapsula datos sensibles en un `Vec<u8>`
- Acceso solo lectura al buffer (`as_slice`)
- Limpieza best-effort de memoria usando:
  - `write_volatile`
  - `compiler_fence`
- El usuario del crate no interactúa con punteros crudos

## ▶ Ejecución
```bash
cargo fmt
cargo clippy
cargo audit
cargo test
🧩 Cómo utilizarlo
use secure_adapter_project::{LegacyAdapter, SecureData, SecurityLevel};

let adapter = LegacyAdapter::new();

let secure = SecureData::new(
    b"FortisCode".to_vec(),
    SecurityLevel::High,
    "demo-secret",
);

let encrypted = adapter.encrypt_with_legacy(&secure).unwrap();

⚙ Cómo funciona
La API legacy expone la función:

c_api_encrypt(data: *const u8, len: usize)
El LegacyAdapter:

Valida precondiciones

Obtiene puntero + tamaño desde SecureData

Ejecuta la llamada unsafe de forma controlada

El cifrado es simulado (XOR con 0xAA) para permitir pruebas determinísticas.

Estado: Semana 3 (en progresp)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC