# 🟦 RUST-12 — Flyweight de Certificados

# RUST-12 — Flyweight de Certificados

## 📌 Descripción
Implementación del patrón Flyweight para gestionar de manera eficiente miles de certificados digitales compartiendo partes comunes (estado intrínseco) como el emisor, país y algoritmo de firma para ahorrar memoria.

## 🧱 Funcionalidades
- `IssuerData`: estado intrínseco compartido (CA name, country, algorithm).
- `IndividualCertificate`: estado extrínseco único (serial_number, public_key, subject).
- `CertificateFactory`: fábrica que comparte `Arc<IssuerData>` evitando duplicados.
- `CertificateStore`: simulación de almacenamiento en memoria de certificados.

## 🔐 Eficiencia de Memoria
- Uso de `Arc<IssuerData>` para compartir el estado intrínseco entre múltiples certificados.
- El Factory almacena emisores únicos en un `HashMap<String, Arc<IssuerData>>`.

🧩 Cómo utilizarlo
use certificate_flyweight::{CertificateFactory};


let mut factory = CertificateFactory::new();


// Crear certificados
let cert1 = factory.create_certificate("FortisCA", "BO", "RSA-2048", 1, vec![1,2,3], "CN=user1".to_string());
let cert2 = factory.create_certificate("FortisCA", "BO", "RSA-2048", 2, vec![4,5,6], "CN=user2".to_string());


// Ambos comparten el IssuerData
assert!(std::sync::Arc::ptr_eq(cert1.issuer(), cert2.issuer()));
⚙ Cómo funciona

Se construye una clave única del emisor.

El Factory reutiliza la instancia de IssuerData si ya existe.

Cada certificado contiene su estado único y una referencia compartida al IssuerData.

## ▶ Ejecución
```bash
cargo fmt
cargo clippy
cargo audit
cargo doc
cargo test


Estado: Sprint 03 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC
