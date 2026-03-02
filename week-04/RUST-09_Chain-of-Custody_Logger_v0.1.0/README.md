🟢 README.md (formato FortisCode técnico)
RUST-09 — Chain-of-Custody Logger v0.1.0
Descripción

Microservicio de auditoría con integridad verificable diseñado para entornos de ciberseguridad.
Cada evento se encadena criptográficamente al anterior y se firma digitalmente para impedir alteraciones.

Garantiza no repudio, integridad y detección de manipulación en registros críticos.

Arquitectura
audit_logger/
├── src/
│   ├── core/
│   │   ├── chain.rs
│   │   └── signer.rs
│   ├── db/
│   │   └── audit_repo.rs
│   └── main.rs
└── tests/
    └── integrity_test.rs
Modelo de auditoría

Cada registro contiene:

id

event_type

payload (JSON)

timestamp

prev_hash

current_hash

signature

Encadenamiento:

current_hash = SHA256(event + payload + timestamp + prev_hash)

Firma:

Ed25519(current_hash)
Flujo de seguridad

Evento recibido vía HTTP /event

Enviado por canal async tokio::mpsc

Encadenado con hash previo

Firmado con clave privada Ed25519

Persistido en PostgreSQL

Endpoint /verify valida:

Cadena de hashes

Firmas digitales

Integridad completa

Ejecución
iniciar servicio
cargo run
enviar evento
curl -X POST http://127.0.0.1:8080/event \
-H "Content-Type: application/json" \
-d '{"event_type":"LOGIN_FAIL","payload":{"user":"root"}}'
verificar integridad
curl http://127.0.0.1:8080/verify

respuesta:

{
  "ok": true,
  "checked": N
}
Tests implementados
1. Detección de manipulación

Modificación manual en PostgreSQL rompe cadena → detectado.

2. Consistencia de cadena

100 logs insertados → prev_hash validado.

3. Firma digital

Verificación Ed25519 sobre contenido del log.

Seguridad implementada

SHA-256 hash chaining

Firma Ed25519

Detección de alteración

Persistencia segura

Async logging (no bloqueo)

Verificación criptográfica completa

👨‍💻utor
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC