# ---------- build (static musl) ----------
FROM rust:1.85-bookworm AS builder
WORKDIR /app

# Se copia solo lo permitido
COPY Cargo.toml ./
COPY .cargo/ .cargo/

# Target temporal
RUN mkdir -p src \
 && printf 'fn main(){println!("security_pipeline ok");}\n' > src/main.rs

# Instala toolchain para musl y compila estático
RUN apt-get update \
 && apt-get install -y --no-install-recommends musl-tools ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

# Build release estático (panic=abort + strip=true vienen del Cargo.toml)
RUN cargo generate-lockfile \
 && cargo build --release --target x86_64-unknown-linux-musl

# Crea passwd/group mínimos para que `whoami` muestre "nonroot"
RUN mkdir -p /out/etc \
 && printf 'nonroot:x:10001:10001:nonroot:/nonexistent:/usr/sbin/nologin\n' > /out/etc/passwd \
 && printf 'nonroot:x:10001:\n' > /out/etc/group

# ---------- busybox stage (solo para traer whoami) ----------
FROM busybox:musl AS bb

# ---------- runtime (scratch, no OS packages => 0 CRITICAL) ----------
FROM scratch
WORKDIR /app

# Busybox estático para ejecutar whoami en la prueba
COPY --from=bb /bin/busybox /busybox
COPY --from=builder /out/etc/passwd /etc/passwd
COPY --from=builder /out/etc/group /etc/group

# Binario estático musl
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/security_pipeline /app/security_pipeline

# NO ROOT
USER 10001:10001

ENTRYPOINT ["/app/security_pipeline"]