#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$CRATE_DIR"

MODE="${1:-all}"

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || { echo "[!] Falta comando: $1"; exit 1; }
}

install_cargo_tool_if_missing() {
  local bin="$1"
  local crate="$2"
  if ! command -v "$bin" >/dev/null 2>&1; then
    echo "[*] Instalando $crate..."
    cargo install --locked "$crate"
  fi
}

die() { echo "[!] $*"; exit 1; }

DENY_TMP=""
CARGO_TOML_BAK=""

cleanup() {
  [[ -n "${DENY_TMP:-}" && -f "${DENY_TMP:-}" ]] && rm -f "$DENY_TMP"
  if [[ -n "${CARGO_TOML_BAK:-}" && -f "${CARGO_TOML_BAK:-}" ]]; then
    mv -f "$CARGO_TOML_BAK" "$CRATE_DIR/Cargo.toml"
    rm -f Cargo.lock || true
  fi
}
trap cleanup EXIT

TEMP_SRC_CREATED=0
ensure_temp_target() {
  if [[ ! -d "src" ]]; then
    mkdir -p src
    TEMP_SRC_CREATED=1
  fi
  if [[ ! -f "src/main.rs" ]]; then
    printf 'fn main(){println!("security_pipeline ok");}\n' > src/main.rs
    TEMP_SRC_CREATED=1
  fi
}

remove_temp_target_if_created() {
  if [[ "${TEMP_SRC_CREATED}" -eq 1 ]]; then
    rm -rf src
    TEMP_SRC_CREATED=0
  fi
}

ensure_release_hardening() {
  echo "[*] Verificando hardening en Cargo.toml (panic=abort, strip=true)..."
  grep -qE '^\[profile\.release\]' Cargo.toml || die "Falta [profile.release]"
  grep -qE '^\s*panic\s*=\s*"?abort"?\s*$' Cargo.toml || die "Falta panic = \"abort\""
  grep -qE '^\s*strip\s*=\s*true\s*$' Cargo.toml || die "Falta strip = true"
  echo "    OK"
}

cargo_audit_rustsec() {
  echo "[*] Auditoría de Crates: cargo audit (RustSec)..."
  ensure_temp_target
  install_cargo_tool_if_missing cargo-audit cargo-audit

  rm -f Cargo.lock || true
  cargo generate-lockfile
  cargo audit
  echo "    OK"

  remove_temp_target_if_created
}

make_temp_deny_config() {
  DENY_TMP="$(mktemp)"
  cat > "$DENY_TMP" <<'TOML'
# Config compatible con cargo-deny actual (sin keys removidas)

[advisories]
# "vulnerability/notice/unsound" ya no se configuran como warn/deny aquí; ahora emiten error por defecto.
# Se controla con ignore si necesitas excepciones.
unmaintained = "workspace"
unsound = "all"
yanked = "deny"
ignore = []
[licenses]
confidence-threshold = 0.93
allow = [
  "MIT",
  "Apache-2.0",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "ISC",
  "Zlib",
  "CC0-1.0"
]

# Excepción SOLO para el crate local sin license field
[[licenses.exceptions]]
name = "security_pipeline"
allow = ["MIT"]

[sources]
# Supply-chain: bloquear fuentes no oficiales
unknown-registry = "deny"
unknown-git = "deny"
TOML
}
cargo_deny_supply_chain() {
  echo "[*] Control de Supply-Chain: cargo deny..."
  ensure_temp_target
  install_cargo_tool_if_missing cargo-deny cargo-deny
  make_temp_deny_config

  rm -f Cargo.lock || true
  cargo generate-lockfile

  cargo deny check -c "$DENY_TMP" advisories licenses sources
  echo "    OK"

  remove_temp_target_if_created
}

test_vulnerability_pipeline_must_fail() {
  echo "[*] Test de Vulnerabilidad: inyectando crate vulnerable y esperando fallo de cargo audit..."
  ensure_temp_target
  install_cargo_tool_if_missing cargo-audit cargo-audit

  # Backup Cargo.toml
  CARGO_TOML_BAK="$(mktemp)"
  cp Cargo.toml "$CARGO_TOML_BAK"

  # Garantiza [dependencies]
  grep -qE '^\[dependencies\]' Cargo.toml || printf "\n[dependencies]\n" >> Cargo.toml

  # Inyecta time = "=0.1.45" (RUSTSEC-2020-0071)
  if grep -qE '^\s*time\s*=' Cargo.toml; then
    sed -i -E 's/^\s*time\s*=.*/time = "=0.1.45"/' Cargo.toml
  else
    awk '
      BEGIN{ins=0}
      /^\[dependencies\]$/ && ins==0 { print; print "time = \"=0.1.45\""; ins=1; next }
      {print}
    ' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml
  fi

  rm -f Cargo.lock || true
  cargo generate-lockfile

  set +e
  cargo audit
  rc=$?
  set -e

  if [[ $rc -eq 0 ]]; then
    die "FALLÓ LA PRUEBA: cargo audit debía fallar con time 0.1.45 pero NO falló."
  fi

  echo "    OK: El pipeline falló como se esperaba por dependencia vulnerable."

  # Restaurar Cargo.toml
  mv -f "$CARGO_TOML_BAK" "$CRATE_DIR/Cargo.toml"
  CARGO_TOML_BAK=""

  rm -f Cargo.lock || true
  cargo generate-lockfile >/dev/null 2>&1 || true

  remove_temp_target_if_created
}

docker_build() {
  echo "[*] Docker Security: build de imagen..."
  require_cmd docker
  docker build -t security_pipeline:ci .
  echo "    OK"
}

privilege_test_whoami() {
  echo "[*] Prueba de Privilegios: whoami dentro del contenedor (NO root)..."
  require_cmd docker
  who="$(docker run --rm --entrypoint /busybox security_pipeline:ci whoami)"
  echo "    whoami => $who"
  [[ "$who" != "root" ]] || die "FALLÓ: el contenedor corre como root"
  echo "    OK"
}

binary_strip_check() {
  echo "[*] Verificación binario: sin símbolos innecesarios (strip)..."
  require_cmd docker

  cid="$(docker create security_pipeline:ci)"
  tmpdir="$(mktemp -d)"
  bin="$tmpdir/security_pipeline"
  docker cp "$cid:/app/security_pipeline" "$bin"
  docker rm "$cid" >/dev/null

  if command -v file >/dev/null 2>&1; then
    info="$(file "$bin" || true)"
    echo "    file => $info"
    echo "$info" | grep -qi 'stripped' || die "FALLÓ: el binario no parece estar stripped"
  else
    echo "    (WARN) 'file' no está instalado en tu host; no se pudo validar 'stripped'."
  fi

  if command -v readelf >/dev/null 2>&1; then
    if readelf -S "$bin" | grep -q '\.debug'; then
      die "FALLÓ: se encontraron secciones .debug"
    fi
  else
    echo "    (WARN) 'readelf' no está instalado en tu host; instala binutils para evidencia extra."
  fi

  rm -rf "$tmpdir"
  echo "    OK"
}

scan_image_trivy_zero_critical() {
  echo "[*] Escaneo de Imagen: trivy (0 CRITICAL requerido)..."
  require_cmd trivy
  trivy image --severity CRITICAL --exit-code 1 --no-progress security_pipeline:ci
  echo "    OK: 0 CRITICAL"
}

case "$MODE" in
  all)
    ensure_release_hardening
    cargo_audit_rustsec
    cargo_deny_supply_chain
    test_vulnerability_pipeline_must_fail
    docker_build
    privilege_test_whoami
    binary_strip_check
    scan_image_trivy_zero_critical
    echo "[✓] Tarea RUST-10 completada con todas las pruebas."
    ;;
  audit)
    ensure_release_hardening; cargo_audit_rustsec ;;
  deny)
    ensure_release_hardening; cargo_deny_supply_chain ;;
  test-vuln)
    ensure_release_hardening; test_vulnerability_pipeline_must_fail ;;
  docker)
    ensure_release_hardening; docker_build; privilege_test_whoami; binary_strip_check ;;
  trivy)
    docker_build; scan_image_trivy_zero_critical ;;
  *)
    echo "Uso: $0 {all|audit|deny|test-vuln|docker|trivy}"
    exit 2
    ;;
esac