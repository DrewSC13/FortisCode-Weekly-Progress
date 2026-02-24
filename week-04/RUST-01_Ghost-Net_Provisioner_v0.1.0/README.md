# RUST-01 Ghost-Net Provisioner v0.1.0
**Sprint 04 — Zero-Trust Networking + Hardening Docker (Rust Microservice Gateway)**

Este proyecto implementa una infraestructura de despliegue para un microservicio de ciberseguridad escrito en **Rust** que actúa como **gateway** entre una red expuesta (frontend) y una red interna altamente restrictiva (backend). El objetivo es que **PostgreSQL** y **Redis** sean **invisibles desde el exterior del host**, permitiendo comunicación únicamente por una red interna `backend_net`.

---

## Objetivos de Seguridad

- **Aislamiento de DB/Cache**: PostgreSQL y Redis **no exponen puertos** al host.
- **Zero-Trust a nivel red**: segmentación por redes Docker:
  - `frontend_net`: expuesta (bridge)
  - `backend_net`: interna (`internal: true`)
- **Gateway dual-homed**: el microservicio Rust pertenece a **ambas redes** y es el único punto de entrada.
- **Hardening de contenedor** (gateway):
  - imagen final **distroless** (sin `sh`, sin `curl`, sin package manager)
  - `cap_drop: ALL`
  - `no-new-privileges`
  - filesystem `read_only`
  - `tmpfs` para `/tmp`
- **Mitigación DoS local**: límites de CPU y memoria por contenedor.

---

## Arquitectura (alto nivel)


Internet/Host
|
| 8080/tcp (único puerto expuesto)
v
[gateway: Rust distroless] <-- frontend_net + backend_net
|
| backend_net (internal: true)
+--> [db: Postgres] (sin ports)
+--> [cache: Redis] (sin ports, con requirepass)


**Garantía principal**: DB y Redis NO son accesibles desde `127.0.0.1` del host porque no existen mapeos `ports`.

---

## Estructura del repositorio


ghost_provisioner/
├── deploy/
│ ├── Dockerfile
│ └── docker-compose.yml
├── src/
│ └── main.rs
├── Cargo.toml
├── Cargo.lock
├── .dockerignore
└── README.md


---

## Requisitos

- Docker Engine + Docker Compose (plugin v2)
- Rust toolchain (para desarrollo local opcional)
- Comandos opcionales de prueba: `nc` (netcat), `telnet`

---

## Variables y credenciales (demo)

> ⚠️ Credenciales de ejemplo para laboratorio. En producción usar Secrets (Docker Secrets/Vault).

- PostgreSQL:
  - DB: `ghostnet`
  - USER: `ghost`
  - PASS: `ghost_change_me`

- Redis:
  - PASS: `ghost_cache_change_me`

---

## Build & Deploy

Desde la carpeta `ghost_provisioner/`:

```bash
docker compose -f deploy/docker-compose.yml down -v
docker compose -f deploy/docker-compose.yml up -d --build
docker compose -f deploy/docker-compose.yml ps
