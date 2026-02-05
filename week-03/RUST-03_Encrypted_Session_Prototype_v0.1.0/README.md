# 🟩 RUST-03 — Encrypted Session Prototype  
`README.md` **(ampliado y técnico)**

```markdown
# RUST-03 — Encrypted Session Prototype

## 📌 Descripción General
Este proyecto aplica el **patrón Prototype** para clonar sesiones de usuario de forma segura.

Permite:
- Copiar metadatos de una sesión
- Regenerar automáticamente datos sensibles
- Evitar reutilización de tokens de acceso

---

## 🧠 Problema que resuelve
En arquitecturas distribuidas:
- Las sesiones deben transferirse entre servicios
- Copiar tokens rompe principios de seguridad
- Es necesario clonar estructura, no secretos

Este patrón permite **clonar sin comprometer seguridad**.

---

## 🧱 Componentes

### Trait
- `SessionPrototype`
  - Define el método `spawn_clone()`

### Struct
- `Session`
  - `user_id`
  - `permissions`
  - `session_token`

### Utilidades
- Generación de tokens únicos (simulados)

---

## 🔐 Reglas de Seguridad
- Cada clon genera un `session_token` nuevo
- Los permisos se copian exactamente
- El `user_id` se mantiene
- Nunca se reutiliza el token original

---

## 🧩 Uso Básico
```rust
let master = Session::new("user-1", vec!["read", "write"]);
let clone = master.spawn_clone();

assert_eq!(master.user_id(), clone.user_id());
assert_ne!(master.token(), clone.token());
⚙️ Funcionamiento Interno
spawn_clone() crea una nueva instancia

Copia campos no sensibles

Genera un nuevo token aleatorio

Retorna una sesión completamente independiente

🧪 Pruebas
Unitarias
Verifica que user_id se conserva

Verifica que permissions son idénticos

Verifica que el token cambia

Integración
Crea una sesión maestra

Clona 3 veces

Asegura que las 4 sesiones tienen tokens únicos

▶️ Ejecución
cargo fmt
cargo clippy
cargo test
Estado: Semana 03 (en progreso)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC