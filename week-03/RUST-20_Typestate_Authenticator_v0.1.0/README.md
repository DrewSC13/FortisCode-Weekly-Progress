# 🟦 RUST-20 — Typestate Authenticator

# RUST-20 — Typestate Authenticator

## 📌 Descripción
Implementación del patrón **Typestate** para controlar acceso a operaciones sensibles mediante el sistema de tipos en Rust.
Un `User<Unauthenticated>` es un tipo distinto a un `User<Authenticated>`, por lo que el acceso a datos protegidos no existe para el usuario no autenticado.

## 🧱 Funcionalidades
- Estados como tipos:
  - `Unauthenticated`
  - `Authenticated { token: String }`
- Estructura genérica:
  - `User<S> { name: String, state: S }`
- Transición segura:
  - `User<Unauthenticated>::authenticate(password: &str) -> Result<User<Authenticated>, String>`
- API protegida por diseño:
  - `User<Authenticated>::view_vault_data(&self)`

## 🔐 Seguridad por Diseño (Typestate)
- No existe el método `view_vault_data()` para `User<Unauthenticated>`.
- La transición consume `self`, forzando a que el usuario anterior no pueda reutilizarse.
- Se incluye un `doctest compile_fail` que valida que el acceso indebido ni siquiera compila.

🧩 Cómo utilizarlo
use typestate_auth::User;

let user = User::new("Claudio");

// Esto NO compila por diseño (typestate):
// let _ = user.view_vault_data();

let authed = user.authenticate("fortis123").unwrap();
let data = authed.view_vault_data();

assert!(data.contains("VAULT_DATA"));

⚙ Cómo funciona

Se modela el estado como un tipo (S) en User<S>.

authenticate() consume User<Unauthenticated> y retorna un nuevo User<Authenticated>.

Solo el tipo autenticado expone la API segura, eliminando la necesidad de flags como is_authenticated.

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
