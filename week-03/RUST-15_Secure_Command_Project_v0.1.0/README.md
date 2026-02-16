# 🟦 RUST-15 — Secure Command

# RUST-15 — Secure Command

## 📌 Descripción
Implementación del patrón **Command** para encapsular operaciones sensibles (ej. rotar claves, limpiar buffers) de forma que puedan **encolarse**, **auditarlas** y mantener **historial**.

## 🧱 Funcionalidades
- Trait `SecurityCommand` con `execute(&mut self) -> Result<(), String>`
- `RotateKeysCommand` → rota una clave dentro del estado del sistema
- `WipeSensitiveDataCommand` → sobrescribe un buffer con ceros
- `CommandController` (Invoker) → ejecuta comandos en secuencia y guarda historial

## 🔐 Seguridad de Memoria
- Limpieza de datos sensibles mediante sobrescritura explícita del buffer
- Encapsulamiento de acciones críticas en comandos ejecutables y auditables
- Historial de operaciones para trazabilidad

🧩 Cómo utilizarlo
use secure_command_project::{CommandController, SystemState};
use secure_command_project::commands::{RotateKeysCommand, WipeSensitiveDataCommand};

let mut controller = CommandController::new();
let mut state = SystemState::new("k1".to_string());

controller.enqueue(Box::new(RotateKeysCommand::new(&mut state, "k2".to_string())));

let mut wipe = WipeSensitiveDataCommand::new();
wipe.attach_buffer(vec![1,2,3,4]);
controller.enqueue(Box::new(wipe));

controller.execute_all().unwrap();
assert_eq!(state.current_key(), "k2");
assert_eq!(controller.history_len(), 2);

⚙ Cómo funciona

Cada operación sensible se representa como un comando con execute.

El CommandController actúa como invoker:

encola comandos

los ejecuta en orden

registra el historial exacto de operaciones realizadas

Estado: Semana 03 (completado)
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
