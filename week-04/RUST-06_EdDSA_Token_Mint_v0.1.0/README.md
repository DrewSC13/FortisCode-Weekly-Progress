# RUST-06 – EdDSA Token Mint v0.1.0

Sistema seguro de emisión y validación de JWT utilizando firmas de curva elíptica **Ed25519 (EdDSA)** diseñado para arquitecturas distribuidas de microservicios.

---

## 🎯 Objetivo

Eliminar el riesgo de claves compartidas (HS256) en entornos distribuidos.

Arquitectura segura:

- Servicio de identidad (Auth) → Firma tokens con clave privada
- Microservicios → Validan tokens únicamente con clave pública
- Redis → Permite rotación dinámica de claves públicas

---

## 🔐 Seguridad Implementada

- Algoritmo: **EdDSA (Ed25519)**
- Clave privada nunca compartida
- Validación estricta:
  - `iss` obligatorio
  - `exp` obligatorio
  - Algoritmos simétricos deshabilitados
  - `none` deshabilitado
- Rotación de clave pública vía Redis
- Validación de firma criptográfica fuerte

---

## 📁 Estructura


token_mint/
├── keys/
│ ├── private.pem
│ └── public.pem
├── src/
│ ├── auth/
│ │ ├── signer.rs
│ │ └── verifier.rs
│ └── main.rs
└── tests/
└── jwt_test.rs


---

## ⚙️ Variables de Entorno

| Variable | Descripción |
|-----------|-------------|
| TOKEN_ISS | Emisor del token |
| TOKEN_TTL_SECONDS | Tiempo de expiración |
| REDIS_URL | URL de Redis |
| REDIS_PUBKEY_KEY | Clave donde se guarda la pubkey |

---

## 🚀 Uso

### Emitir Token


cargo run -- mint user-123


### Validar Token


cargo run -- verify <jwt>


---

## 🔄 Rotación de Clave Pública

1. Levantar Redis:


docker run -d --name tm_redis -p 6379:6379 redis:7-alpine


2. Exportar variables:


export REDIS_URL="redis://127.0.0.1:6379"
export REDIS_PUBKEY_KEY="token_mint:pubkey"


3. Subir clave pública:


cargo run -- push-pubkey-redis


---

## 🧪 Tests de Seguridad

### ✅ Firma Cruzada Rechazada
Un token firmado con otra clave privada es rechazado.

### ✅ ExpiredSignature
Token con `exp` en el pasado es rechazado.

### ✅ Caja Negra
Un microservicio con solo la clave pública:
- Puede leer payload
- No puede modificar el contenido sin romper la firma

---

## 🛡 Modelo de Seguridad


Auth Service
|
| (Private Key)
v
JWT firmado
|
v
Microservicios
|
| (Public Key)
v
Verificación segura


---

## 📊 Validaciones

- cargo fmt ✅
- cargo clippy ✅
- cargo audit ✅
- cargo test (3/3) ✅

---

## 📌 Cumple con

- Seguridad en arquitecturas distribuidas
- Eliminación de secretos compartidos
- Firma asimétrica robusta
- Preparado para rotación de claves
- JWT estándar con EdDSA

---

## 🧠 Autor
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC