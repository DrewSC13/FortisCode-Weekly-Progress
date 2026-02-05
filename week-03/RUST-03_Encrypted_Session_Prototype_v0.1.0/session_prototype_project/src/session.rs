use crate::SessionPrototype;
use crate::crypto_utils::generate_session_token;

/// Representa una sesión segura.
///
/// - `user_id`: identificador del usuario
/// - `permissions`: permisos/roles del usuario
/// - `session_token`: token temporal (NO debe copiarse en clones)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    user_id: String,
    permissions: Vec<String>,
    session_token: String,
}

impl Session {
    /// Crea una sesión nueva con token generado.
    pub fn new(user_id: impl Into<String>, permissions: Vec<String>) -> Self {
        Self {
            user_id: user_id.into(),
            permissions,
            session_token: generate_session_token(),
        }
    }

    /// Métodos públicos para lectura.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn permissions(&self) -> &[String] {
        &self.permissions
    }

    pub fn token(&self) -> &str {
        &self.session_token
    }
}

impl SessionPrototype for Session {
    /// Clona la sesión manteniendo metadatos,regenera el token.
    fn spawn_clone(&self) -> Self {
        Self {
            user_id: self.user_id.clone(),
            permissions: self.permissions.clone(),
            session_token: generate_session_token(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_clone_keeps_same_user_id() {
        let original = Session::new("user-123", vec!["read".into(), "write".into()]);
        let cloned = original.spawn_clone();

        assert_eq!(original.user_id(), cloned.user_id());
    }

    #[test]
    fn spawn_clone_keeps_permissions_equal() {
        let original = Session::new("user-123", vec!["read".into(), "write".into()]);
        let cloned = original.spawn_clone();

        assert_eq!(original.permissions(), cloned.permissions());
    }

    #[test]
    fn spawn_clone_generates_new_token() {
        let original = Session::new("user-123", vec!["read".into()]);
        let cloned = original.spawn_clone();

        assert_ne!(original.token(), cloned.token());
    }
}
