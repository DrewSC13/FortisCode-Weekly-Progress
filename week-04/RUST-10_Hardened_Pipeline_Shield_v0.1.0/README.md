# RUST-10 Hardened Pipeline Shield v0.1.0

Micro-entorno de construcción segura enfocado en la detección temprana de vulnerabilidades en dependencias Rust y hardening del binario final dentro de un contenedor Docker Zero-Trust.

Implementa auditoría de supply-chain, validación de licencias, binario hardened y ejecución non-root para prevenir ataques modernos basados en dependencias maliciosas.

---

## Objetivo

Configurar un pipeline de seguridad que:

- Audite automáticamente dependencias Rust
- Detecte crates vulnerables (RustSec)
- Bloquee dependencias sospechosas o licencias no permitidas
- Endurezca el binario final eliminando símbolos innecesarios
- Ejecute el contenedor sin privilegios root
- Verifique imagen Docker sin vulnerabilidades críticas

---

## Arquitectura

security_pipeline/
├── .cargo/
│   └── config.toml          → Flags de compilación segura
├── scripts/
│   └── scan_deps.sh         → Script automatizado de auditoría
├── Dockerfile               → Contenedor hardened non-root
└── Cargo.toml               → Configuración de release segura

---

## Componentes de Seguridad

### 1. Auditoría de dependencias (RustSec)
Se ejecuta automáticamente:

cargo audit

Detecta crates con vulnerabilidades conocidas.

---

### 2. Control de Supply-Chain
Implementado con:

cargo deny

Bloquea:
- Licencias no permitidas
- Dependencias sospechosas
- Fuentes no oficiales

---

### 3. Hardening del binario Rust

Configuración en Cargo.toml:

[profile.release]
panic = "abort"
strip = true
lto = true
codegen-units = 1

Resultado:
- Binario sin símbolos debug
- Tamaño reducido
- Menor superficie de ataque
- Compilación optimizada y reproducible

---

### 4. Hardening del contenedor Docker

Seguridad aplicada:

- Multi-stage build
- Binario estático (musl)
- Usuario non-root
- Sin privilegios elevados
- Sin herramientas innecesarias
- Capabilities eliminadas
- no-new-privileges

Verificación:

docker image inspect security_pipeline:ci
User=10001:10001

---

## Script de Seguridad Automatizado

Ejecutar pipeline completo:

./security_pipeline/scripts/scan_deps.sh all

Opciones disponibles:

audit       → cargo audit
deny        → cargo deny
test-vuln   → test con crate vulnerable
docker      → build hardened container
trivy       → escaneo imagen final
all         → pipeline completo

---

## Validaciones implementadas

### Test de vulnerabilidad
Se inyecta crate vulnerable (time 0.1.45):

Resultado esperado:
cargo audit falla → pipeline detecta vulnerabilidad

---

### Prueba non-root
Dentro del contenedor:

whoami → nonroot  
UID ≠ 0  
sin permisos root  

Intentos:
mkdir /root → FAIL  
touch /etc/shadow → FAIL  

---

### Hardening del binario
Verificado:

- stripped
- sin debug symbols
- sin symtab
- PIE activo
- RELRO activo
- NX activo
- BIND_NOW activo

---

### Escaneo de imagen Docker

trivy image security_pipeline:ci

Resultado:
0 vulnerabilidades críticas  
0 secrets  
imagen limpia

---

## Resultado

Pipeline hardened funcional para entornos DevSecOps y Zero-Trust:

- Supply-chain protegida
- Binario hardened
- Contenedor non-root
- Auditoría automática
- Escaneo de seguridad completo

---

## Autor
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC