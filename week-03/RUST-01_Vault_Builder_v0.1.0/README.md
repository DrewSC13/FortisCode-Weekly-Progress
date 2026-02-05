🟩 RUST-01 — Vault Builder

README.md (versión completa y sólida)

# RUST-01 — Vault Builder

## 📌 Descripción General
Este proyecto implementa el **patrón Builder** para construir una **Bóveda Digital (Vault)** de forma segura y controlada.

El objetivo es garantizar que una bóveda **no pueda existir en un estado inseguro**, aplicando validaciones estrictas antes de su creación final.

El diseño sigue principios de:
- Encapsulación
- Validación temprana
- APIs seguras por defecto

---

## 🧠 Problema que resuelve
En sistemas de seguridad:
- Configuraciones inválidas pueden comprometer datos
- No todos los algoritmos son válidos para todos los niveles
- Los secretos no deben exponerse directamente

Este Builder evita esos riesgos **antes** de crear la instancia.

---

## 🧱 Componentes Principales

### Enums de Seguridad
- `EncryptionLevel`: `Low | Medium | High`
- `CipherAlgorithm`: `AES128 | AES256 | ChaCha20`

### Estructuras
- `Vault`: estructura final **con campos privados**
- `VaultBuilder`: intermediario responsable de validar la configuración

---

## 🔐 Reglas de Seguridad Implementadas
- `EncryptionLevel::High` **no permite** `AES128`
- `secret_key` es obligatoria y no puede estar vacía
- `timeout` tiene valor por defecto seguro (`300s`)
- El `Vault` solo puede construirse vía el Builder

---

## 🧩 Uso Básico
```rust
use vault_system::{Vault, EncryptionLevel, CipherAlgorithm};

let vault = Vault::builder()
    .level(EncryptionLevel::High)
    .algorithm(CipherAlgorithm::AES256)
    .timeout(300)
    .key(vec![1,2,3,4,5,6,7,8])
    .build()
    .unwrap();

println!("Vault ID: {}", vault.get_id());

⚙️ Funcionamiento Interno

El usuario configura el VaultBuilder

Cada método encadenado guarda valores en Option<T>

build():

Verifica presencia de todos los campos críticos

Aplica validaciones de seguridad

Genera un id único

Solo si todo es válido se crea el Vault

🧪 Pruebas
Unitarias

Construcción válida

Validación de seguridad (High + AES128 → error)

Timeout por defecto

Integración

Simula uso real desde una crate externa

Verifica encapsulación (no acceso a campos privados)

▶️ Ejecución
cargo fmt
cargo clippy
cargo test


Estado: Semana 03 (en progreso)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC