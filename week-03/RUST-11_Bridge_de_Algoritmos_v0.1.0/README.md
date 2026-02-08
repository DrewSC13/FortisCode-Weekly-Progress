# 🟦 RUST-11 — Bridge de Algoritmos

# RUST-11 — Bridge de Algoritmos

## 📌 Descripción
Implementación del patrón Bridge para separar la abstracción de operaciones criptográficas (alto nivel) de la implementación técnica (backend), permitiendo intercambiar algoritmos/implementaciones (OpenSSL simulado vs Rust nativo) sin afectar al cliente.

## 🧱 Funcionalidades
- Trait `CryptoBackend` con:
  - `raw_encrypt`
  - `raw_decrypt`
- Backends:
  - `OpenSSLBackend` (simulado)
  - `NativeRustBackend` (nativo simulado)
- Abstracciones de alto nivel:
  - `DigitalSignature` (firma/verificación)
  - `FileEncryption` (cifrado/descifrado)
- Inyección e intercambio de backend en tiempo de ejecución

## 🔐 Seguridad de Memoria
- Uso de `Box<dyn CryptoBackend>` para desacoplamiento seguro
- Sin uso de `unsafe`
- Procesamiento por referencia (`&[u8]`) evitando copias innecesarias fuera del flujo

## ▶ Ejecución
```bash
cargo fmt
cargo clippy
cargo audit
cargo test
🧩 Cómo utilizarlo
use crypto_bridge::{DigitalSignature, OpenSSLBackend, NativeRustBackend};

let msg = b"mensaje";

// Firma con OpenSSLBackend
let ds1 = DigitalSignature::new(Box::new(OpenSSLBackend));
let sig1 = ds1.sign(msg);
assert!(ds1.verify(msg, &sig1));

// Firma con NativeRustBackend
let ds2 = DigitalSignature::new(Box::new(NativeRustBackend));
let sig2 = ds2.sign(msg);
assert!(ds2.verify(msg, &sig2));
⚙ Cómo funciona
DigitalSignature y FileEncryption representan la lógica de alto nivel.

Cada abstracción delega el trabajo técnico a CryptoBackend.

Los backends implementan transformaciones reversibles (simuladas) para permitir pruebas determinísticas.

El cliente puede cambiar backends en runtime sin modificar la abstracción.

Estado: Sprint 03 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC
