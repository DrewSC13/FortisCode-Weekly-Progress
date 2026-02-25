🔐 RUST-04 — Argon2 Identity Guard v0.1.0

Microservicio de gestión de identidades con hashing de contraseñas Argon2id resistente a ataques de fuerza bruta con GPU/ASIC.
Implementado en Rust como servicio seguro de registro y autenticación.

📌 Descripción

El almacenamiento de contraseñas es el punto más crítico de cualquier sistema.
Este microservicio implementa un Identity Guard que:

Convierte contraseñas en hashes seguros con Argon2id

Genera salt criptográfico único por usuario

Permite verificar credenciales sin exponer contraseñas

Ajusta la dureza del algoritmo según política de seguridad

Incluye benchmark automático para reforzar resistencia

🧠 Objetivos de seguridad

✔ Hashing moderno resistente a GPU/ASIC
✔ Salt único criptográficamente seguro
✔ Configuración dinámica de dureza
✔ Benchmark automático de seguridad
✔ API REST + CLI
✔ Arquitectura modular sin lib.rs (según requerimiento de tarea)

🏗 Arquitectura del proyecto
identity_guard/
├── src/
│   ├── core/
│   │   ├── mod.rs
│   │   ├── hasher.rs        # Implementación Argon2id
│   │   └── models.rs        # Modelo User
│   └── main.rs              # API REST + CLI
│
├── tests/
│   └── crypto_test.rs       # Tests de seguridad
│
└── Cargo.toml

🔐 Algoritmo criptográfico

Se utiliza:

Argon2id (Password Hashing Competition Winner)
Crate: argon2 (RustCrypto)

Parámetros mínimos de seguridad
Parámetro	Valor mínimo
Memoria	≥ 64 MB
Iteraciones	≥ 3
Paralelismo	núcleos CPU disponibles
Salt	automático y aleatorio
Output	hash codificado PHC

⚙️ Funciones principales
Hash de contraseña
hash_password(password: &str) -> Result<String, Error>

Genera hash Argon2id con salt automático.

Verificación
verify_password(hash: &str, password: &str) -> bool

Compara contraseña en texto plano contra hash almacenado.

🧪 Benchmark de seguridad

El sistema mide el tiempo de hash.

Si el tiempo es < 100ms:

incrementa memoria Argon2

endurece parámetros automáticamente

Esto dificulta ataques de fuerza bruta.

🌐 API REST
Health check
GET /health

Respuesta:

OK
Registro de usuario
POST /register

Body:

{
  "username": "admin",
  "password": "S3cret!"
}
Login
POST /login

Body:

{
  "username": "admin",
  "password": "S3cret!"
}
Benchmark
GET /benchmark

Retorna métricas de seguridad del hashing.

🖥 CLI integrada

Ejecutar en modo CLI:

Hash manual
cargo run -- hash password123
Verificar
cargo run -- verify <hash> password123
Benchmark
cargo run -- benchmark
Modo API
cargo run
🧪 Tests implementados

Archivo:

tests/crypto_test.rs
✔ Test de consistencia

verify correcto → true

verify incorrecto → false

✔ Test de unicidad

misma password → hashes distintos por salt

✔ Test benchmark seguridad

verifica dureza mínima >100ms

Ejecutar:

cargo test

🛡 Hardening criptográfico aplicado

Argon2id recomendado por OWASP

Salt aleatorio por usuario

Sin almacenamiento de passwords en claro

Ajuste automático de dureza

Configurable por CPU container

Preparado para microservicio Zero-Trust

📊 Validación técnica
cargo fmt
cargo clippy
cargo audit
cargo doc
cargo test

✔ Todos OK
✔ Sin vulnerabilidades críticas
✔ Tests de seguridad pasando
✔ Documentación generada

👨‍💻 Autor
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC

🚀 Versión
v0.1.0 — Argon2 Identity Guard inicial
