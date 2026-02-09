//! Patrón Strategy: intercambio dinámico de métodos de autenticación
//! (Password, 2FA, Biometría) según contexto de riesgo.

/// Trait Strategy: define el método authenticate(&self, credentials: &str) -> bool.
pub trait AuthStrategy {
    fn authenticate(&self, credentials: &str) -> bool;
}

#[path = "strategies/password.rs"]
pub mod password;

#[path = "strategies/mfa.rs"]
pub mod mfa;

#[path = "strategies/biometric.rs"]
pub mod biometric;

/// LoginManager contiene una estrategia intercambiable en runtime.
pub struct LoginManager {
    strategy: Box<dyn AuthStrategy>,
}

impl LoginManager {
    pub fn new(strategy: Box<dyn AuthStrategy>) -> Self {
        Self { strategy }
    }

    /// Permite cambiar el método de login sobre la marcha.
    pub fn set_strategy(&mut self, strategy: Box<dyn AuthStrategy>) {
        self.strategy = strategy;
    }

    /// Ejecuta el login usando la estrategia actual.
    pub fn login(&self, credentials: &str) -> bool {
        self.strategy.authenticate(credentials)
    }
}

pub use biometric::BiometricAuth;
pub use mfa::TwoFactorAuth;
pub use password::PasswordAuth;
