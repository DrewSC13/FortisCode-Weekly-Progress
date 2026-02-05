//! Funciones criptográficas simuladas.

use std::{thread, time::Duration};

/// Simula un descifrado pesado.
/// - `sleep` para simular costo.
/// - “Descifra” removiendo el prefijo `ENCRYPTED:`.
pub fn heavy_decrypt_simulated(encrypted: &[u8]) -> String {
    // Simula que cuesta (p.ej. KDF + decrypt + parsing PEM)
    thread::sleep(Duration::from_millis(80));

    let raw = String::from_utf8_lossy(encrypted);

    raw.strip_prefix("ENCRYPTED:").unwrap_or(&raw).to_string()
}
