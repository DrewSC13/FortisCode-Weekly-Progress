//! RUST-20 — Typestate Authenticator (v0.1.0)
//!
//! Typestate: un usuario no autenticado es un tipo distinto a uno autenticado.
//! El acceso a métodos sensibles se controla por el sistema de tipos.
//!
//! ## Prueba de diseño (no compila)
//! El método `view_vault_data()` NO existe para `User<Unauthenticated>`.
//!
//! ```compile_fail
//! use typestate_auth::User;
//!
//! let user = User::new("Claudio");
//! // Esto NO debe compilar: método solo para autenticados.
//! let _ = user.view_vault_data();
//! ```

#[path = "auth_logic.rs"]
pub mod auth_logic;

#[path = "secure_api.rs"]
pub mod secure_api;

pub use auth_logic::{TOKEN_OK, VALID_PASSWORD};

/// Estado: No autenticado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unauthenticated;

/// Estado: Autenticado (contiene token).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authenticated {
    pub(crate) token: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User<S> {
    pub(crate) name: String,
    pub(crate) state: S,
}

impl User<Unauthenticated> {
    /// Crea un usuario no autenticado.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: Unauthenticated,
        }
    }

    /// Nombre del usuario.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Autentica consumiendo `self` y retornando un usuario autenticado.
    pub fn authenticate(self, password: &str) -> Result<User<Authenticated>, String> {
        auth_logic::authenticate_user(self, password)
    }
}

impl User<Authenticated> {
    /// Nombre del usuario.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Devuelve el token asociado al estado autenticado.
    pub fn token(&self) -> &str {
        &self.state.token
    }

    /// API exclusiva para usuarios autenticados.
    pub fn view_vault_data(&self) -> String {
        secure_api::view_vault_data(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unitarias: verificar que authenticate con clave correcta devuelve User<Authenticated>.
    #[test]
    fn authenticate_with_correct_password_returns_authenticated_user() {
        let user = User::new("Claudio");
        let authed = user
            .authenticate(VALID_PASSWORD)
            .expect("must authenticate");
        assert_eq!(authed.name(), "Claudio");
        assert_eq!(authed.token(), TOKEN_OK);
    }
}
