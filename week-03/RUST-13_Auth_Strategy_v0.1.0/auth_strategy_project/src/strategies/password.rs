use crate::AuthStrategy;

/// Estrategia concreta: PasswordAuth
/// Valida contra un hash almacenado.
pub struct PasswordAuth {
    stored_hash: u64,
}

impl PasswordAuth {
    pub fn new(stored_hash: u64) -> Self {
        Self { stored_hash }
    }

    /// Helper: crea una estrategia almacenando el hash de un password conocido.
    pub fn from_password(password: &str) -> Self {
        Self {
            stored_hash: fnv1a64(password.as_bytes()),
        }
    }
}

impl AuthStrategy for PasswordAuth {
    fn authenticate(&self, credentials: &str) -> bool {
        fnv1a64(credentials.as_bytes()) == self.stored_hash
    }
}

// Hash FNV-1a 64 (determinístico, simple, sin crates externas)
fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 14695981039346656037;
    const PRIME: u64 = 1099511628211;

    let mut hash = OFFSET;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unitarias: PasswordAuth devuelve false si la contraseña es incorrecta.
    #[test]
    fn password_auth_returns_false_on_wrong_password() {
        let auth = PasswordAuth::from_password("correct-password");
        assert!(!auth.authenticate("wrong-password"));
    }
}
