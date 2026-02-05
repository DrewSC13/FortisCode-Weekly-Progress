//! Lazy Key Loader
//!
//! Implementa un cargador de claves privadas que solo descifra y sube a memoria
//! la clave cuando se necesita por primera vez (patron Lazy).

pub mod crypto;
pub mod store;

use std::sync::{OnceLock, atomic::AtomicUsize, atomic::Ordering};

/// Cargador lazy de clave privada.
///
/// - `encrypted_key`: dato crudo cifrado (simulado).
/// - `decrypted_key`: cache de solo lectura con 'OnceLock<String>,
#[derive(Debug)]
pub struct KeyLoader {
    encrypted_key: Vec<u8>,
    decrypted_key: OnceLock<String>,
    // Contador/ flag para tests: cuantas veces se ejecuto el "descifrado pesado".
    decrypt_calls: AtomicUsize,
}

impl KeyLoader {
    /// Crea un KeyLoader desde un "almacen frio".
    pub fn from_store(store: &store::ColdStore) -> Self {
        Self {
            encrypted_key: store.read_encrypted_key(),
            decrypted_key: OnceLock::new(),
            decrypt_calls: AtomicUsize::new(0),
        }
    }

    /// Metodo privado: simula un descifrado costoso.
    ///
    /// - Incrementa contador para que os tests confirmen ejecucion
    /// - Usa `sleep` para simular costo.
    fn heavy_decryption(&self) -> String {
        self.decrypt_calls.fetch_add(1, Ordering::SeqCst);
        crypto::heavy_decrypt_simulated(&self.encrypted_key)
    }

    /// Acceso Lazy a la clave descifrada.
    ///
    /// - Si ya esta en memoria (OnceLock lleno), retorna inmediatamente.
    /// - Si no, realiza heavy_decryption() UNA sola vezz y cachea el resultado.
    ///
    /// Seguridad/lectura:
    /// - Retorna `&str` (referencia), no entrega propiedad utable del `String`.
    pub fn get_key(&self) -> &str {
        self.decrypted_key
            .get_or_init(|| self.heavy_decryption())
            .as_str()
    }

    /// Solo para tests: cuantas veces se llamo al proceso de descifrado.
    pub fn decrypt_call_count(&self) -> usize {
        self.decrypt_calls.load(Ordering::SeqCst)
    }

    /// Solo para debug/tests: saber si ya esta cargada.
    pub fn is_loaded(&self) -> bool {
        self.decrypted_key.get().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn before_get_key_decryption_has_not_run() {
        let store = store::ColdStore::new(b"ENCRYPTED:my-private-key");
        let loader = KeyLoader::from_store(&store);

        // Antes de llamar get_key, no debe haberse ejecutado heavy_decryption
        assert_eq!(loader.decrypt_call_count(), 0);
        assert!(!loader.is_loaded());
    }

    #[test]
    fn get_key_runs_decryption_only_once() {
        let store = store::ColdStore::new(b"ENCRYPTED:my-private-key");
        let loader = KeyLoader::from_store(&store);

        let k1 = loader.get_key().to_string();
        let k2 = loader.get_key().to_string();

        assert_eq!(k1, "my-private-key");
        assert_eq!(k2, "my-private-key");

        // Debe ejecutarse solo una vez.
        assert_eq!(loader.decrypt_call_count(), 1);
        assert!(loader.is_loaded());
    }
}
