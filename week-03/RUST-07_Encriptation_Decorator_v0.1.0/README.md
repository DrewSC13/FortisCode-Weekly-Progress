# 🟦 RUST-07 — Encryption Decorator

# RUST-07 — Encryption Decorator

## 📌 Descripción
Implementación del patrón Decorator en Rust utilizando Traits para aplicar capas de procesamiento sobre un flujo de datos, permitiendo añadir cifrado y codificación de forma dinámica.

## 🧱 Funcionalidades
- Trait base `SecureWriter`
- Componente base `PlainTextWriter`
- Decorador `AesDecorator`
- Decorador `Base64Decorator`
- Composición dinámica de capas tipo “cebolla”

## 🔐 Seguridad de Memoria
- El procesamiento se realiza sobre `String` sin uso de punteros crudos
- Cada decorador encapsula su lógica sin exponer detalles internos
- El flujo de datos se controla únicamente mediante el trait `SecureWriter`

## ▶ Ejecución
```bash
cargo fmt
cargo clippy
cargo audit
cargo test

🧩 Cómo utilizarlo
use crypto_decorator_project::{PlainTextWriter, SecureWriter, AesDecorator, Base64Decorator};

let logger = Base64Decorator(AesDecorator(PlainTextWriter));

let result = logger.write("mensaje-secreto");

⚙ Cómo funciona
PlainTextWriter devuelve el mensaje original.

AesDecorator recibe un SecureWriter, cifra el resultado y lo devuelve.

Base64Decorator recibe un SecureWriter, aplica codificación Base64 al resultado.

Las capas pueden apilarse dinámicamente siguiendo el patrón Decorator.

Estado: Sprint 03 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC
