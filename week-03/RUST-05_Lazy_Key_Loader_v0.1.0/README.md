# 🟩 RUST-05 — Lazy Key Loader  

# RUST-05 — Lazy Key Loader

## 📌 Descripción General
Implementación del patrón **Lazy Initialization** para cargar claves privadas solo cuando son necesarias.

Reduce significativamente la **superficie de ataque en memoria RAM**.

---

## 🧠 Problema que resuelve
- Cargar claves al inicio es riesgoso
- Los secretos deben permanecer cifrados
- Solo deben descifrarse cuando se usan

Este proyecto simula un **almacén frío** y carga segura.

---

## 🧱 Componentes

### `KeyLoader`
- `encrypted_key`
- `OnceLock<String>` para la clave descifrada

### `crypto.rs`
- Simulación de descifrado pesado (`sleep`)

### `store.rs`
- Simulación de almacenamiento frío

---

## 🔐 Garantías de Seguridad
- Descifrado ocurre **una sola vez**
- La clave nunca se vuelve a generar
- Acceso posterior es inmediato
- Clave solo lectura tras carga

---

## 🧩 Uso Básico
```rust
let key = loader.get_key();
⚙️ Funcionamiento Interno
get_key() revisa OnceLock

Si está vacío:

Ejecuta descifrado pesado

Guarda el resultado

Si ya existe:

Retorna referencia directa

No hay mutación posterior

🧪 Pruebas
Unitarias
Verifica que no se descifra antes del acceso

Verifica ejecución única

Integración
Primera llamada lenta

Segunda llamada instantánea

Clave correcta en ambos casos

▶️ Ejecución
cargo fmt
cargo clippy
cargo test
Estado: Semana 03 (en progreso)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC