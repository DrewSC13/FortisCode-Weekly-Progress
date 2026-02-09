# 🟦 RUST-13 — Auth Strategy

# RUST-13 — Auth Strategy

## 📌 Descripción
Implementación del patrón Strategy para permitir el intercambio dinámico de métodos de autenticación (Password, 2FA, Biometría) según el contexto de riesgo, haciendo las estrategias intercambiables en tiempo de ejecución.

## 🧱 Funcionalidades
- Trait `AuthStrategy` con `authenticate(&self, credentials: &str) -> bool`
- Estrategias concretas:
  - `PasswordAuth` (valida contra hash almacenado)
  - `TwoFactorAuth` (simula validación de código de 6 dígitos)
  - `BiometricAuth` (simula firma digital de huella)
- Contexto `LoginManager` con `Box<dyn AuthStrategy>` y cambio dinámico con `set_strategy`

## 🔐 Seguridad de Memoria
- Uso de `Box<dyn AuthStrategy>` para polimorfismo seguro
- Sin uso de `unsafe`
- Evaluación inmutable de credenciales (`&str`)

## ▶ Ejecución
```bash
cargo fmt
cargo clippy
cargo audit
cargo doc
cargo test

🧩 Cómo utilizarlo
use auth_strategy_project::{LoginManager, PasswordAuth, TwoFactorAuth};

let password = PasswordAuth::from_password("my-secret");
let mut manager = LoginManager::new(Box::new(password));

assert!(manager.login("my-secret"));

manager.set_strategy(Box::new(TwoFactorAuth::new("123456")));
assert!(manager.login("123456"));

⚙ Cómo funciona
LoginManager actúa como Contexto y delega la autenticación a la estrategia actual.

Cada estrategia implementa AuthStrategy y define su propio criterio:

Password: compara hash calculado vs hash almacenado

2FA: valida formato de 6 dígitos y coincidencia del token

Biometría: valida la firma simulada

Estado: Sprint 03 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC
