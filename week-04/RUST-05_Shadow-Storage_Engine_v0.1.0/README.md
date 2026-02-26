🔐 RUST-05 — Shadow-Storage Engine (AES-256-GCM Field Encryption)

Microservicio en Rust para cifrado de campos sensibles en PostgreSQL usando AES-256-GCM con autenticación integrada.
Diseñado para cumplir requisitos de GDPR / PCI-DSS: incluso si un atacante obtiene un dump SQL, los datos sensibles permanecen ilegibles.

🎯 Objetivo

Implementar una capa de servicio que:

Cifre datos sensibles antes de almacenarlos en PostgreSQL

Descifre al recuperarlos

Detecte manipulación maliciosa de datos (tampering)

Use cifrado autenticado moderno (AES-256-GCM)

Cumpla principios Zero-Trust y compliance (GDPR/PCI)

🧠 Arquitectura
shadow_storage/
├── src/
│   ├── crypto/
│   │   ├── mod.rs
│   │   └── aes_engine.rs      # Motor AES-256-GCM
│   ├── repository/
│   │   └── user_repo.rs       # Mapeo Struct -> DB cifrada
│   └── main.rs                # CLI/API
└── tests/
    └── encryption_test.rs     # Tests de integridad y seguridad

🔐 Diseño criptográfico
Algoritmo

AES-256-GCM (cifrado autenticado)

Crate: aes-gcm

Propiedades de seguridad

Confidencialidad: AES-256

Integridad: Tag GCM (autenticación)

Nonce único por operación (96 bits)

Key 32 bytes (256 bits)

🔑 Manejo de llave segura

La clave no está en el código.

Se carga desde:

Volumen seguro (archivo)

Variable entorno base64

Docker Secret (compatible)

Generar key
mkdir -p secrets
head -c 32 /dev/urandom > secrets/shadow_key
Usar key desde archivo
SHADOW_KEY_FILE=secrets/shadow_key cargo run -- encrypt "hola"
Usar key base64
export SHADOW_KEY_B64="$(head -c 32 /dev/urandom | base64 -w0)"

🔄 Lógica criptográfica
Encrypt
encrypt(plaintext, key) -> Vec<u8>

Salida:

[ nonce (12B) | tag (16B) | ciphertext ]

Cada cifrado genera:

Nonce aleatorio único

Ciphertext distinto incluso con mismo plaintext

Decrypt
decrypt(blob, key) -> Result<String>

Si un solo bit cambia → error de autenticación.

🧪 Comandos CLI
Cifrar
cargo run -- encrypt "hello"
Descifrar
cargo run -- decrypt HEX_BLOB
Flip bit (test integridad)
cargo run -- flipbit HEX_BLOB
Imprimir schema PostgreSQL
cargo run -- print-schema
🗄️ PostgreSQL Schema
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    email_enc BYTEA NOT NULL,
    national_id_enc BYTEA NOT NULL
);

Los campos sensibles se almacenan como BYTEA cifrado.

🧪 Validaciones de seguridad
1️⃣ Unit tests
cargo test

Valida:

Nonce único

Integridad GCM

Datos ilegibles

2️⃣ Calidad de código
cargo fmt
cargo clippy
cargo audit
cargo doc
cargo test

🔍 Validación de visibilidad (dump DB ilegible)
Levantar PostgreSQL
docker run -d --name shadow_pg \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=shadowdb \
  -p 5432:5432 \
  postgres:16-alpine
Crear tabla
cargo run -- print-schema | psql "postgres://postgres:postgres@127.0.0.1:5432/shadowdb"
Insertar datos cifrados
EMAIL_HEX=$(cargo run -- encrypt "alice@example.com")
NID_HEX=$(cargo run -- encrypt "ID-123456")

psql "postgres://postgres:postgres@127.0.0.1:5432/shadowdb" -c \
"INSERT INTO users(username,email_enc,national_id_enc)
VALUES ('alice', decode('$EMAIL_HEX','hex'), decode('$NID_HEX','hex'));"
Ver DB
SELECT encode(email_enc,'hex') FROM users;

Resultado: datos ilegibles.

🧪 Test de integridad (anti-tampering)
EMAIL_DB_HEX=$(psql ...)

TAMPERED=$(cargo run -- flipbit "$EMAIL_DB_HEX")

cargo run -- decrypt "$TAMPERED"

Resultado esperado:

Error: authentication failed (tampered / tag mismatch)

AES-GCM detecta modificación.

🧪 Test de unicidad (nonce)
cargo run -- encrypt "same"
cargo run -- encrypt "same"

Resultado:

Hash distintos

Nonce aleatorio OK

🛡️ Seguridad aplicada

AES-256-GCM autenticado

Nonce único por operación

Key fuera del código

Compatible Docker Secrets

Protección contra dump DB

Protección contra manipulación

Diseño Zero-Trust

👨‍💻 Autor
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC