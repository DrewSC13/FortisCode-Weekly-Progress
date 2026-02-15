# 🟦 RUST-18 — Policy Visitor

# RUST-18 — Policy Visitor

## 📌 Descripción
Implementación del patrón **Visitor** aplicado a auditorías de políticas de seguridad.

Permite recorrer estructuras como:
- `KeyStorage`
- `AccessLogs`

Sin modificar su código interno, aplicando distintos algoritmos de auditoría de forma desacoplada.

## 🧱 Funcionalidades

### 🎯 Traits principales
- `SecurityVisitor`
- `SecurityElement`

### 🗂 Elementos Visitables
- `KeyStorage`
- `AccessLogs`

### 🕵️ Visitantes Implementados
- `ExpirationChecker` → Detecta claves expiradas
- `SizeAuditor` → Calcula uso total de memoria

🧠 Cómo funciona

El patrón Visitor separa:

Estructura de datos (Elementos)

Algoritmo de operación (Visitantes)

Esto evita modificar las estructuras cada vez que se requiere una nueva política de auditoría.

## 🔁 Doble Despacho
Cada elemento implementa:

```rust
fn accept(&self, visitor: &dyn SecurityVisitor)
Llamando internamente al método correspondiente del visitante.

Esto permite aplicar múltiples auditorías sin modificar los elementos.

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
