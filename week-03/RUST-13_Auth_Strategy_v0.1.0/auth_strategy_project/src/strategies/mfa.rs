use crate::AuthStrategy;

/// Estrategia concreta: TwoFactorAuth
/// simula la validacion de codigo de 6 digitos.
pub struct TwoFactorAuth {
    expected_code: String,
}

impl TwoFactorAuth {
    pub fn new(expected_code: &str) -> Self {
        Self {
            expected_code: expected_code.into(),
        }
    }

    fn is_six_digits(s: &str) -> bool {
        s.len() == 6 && s.chars().all(|c| c.is_ascii_digit())
    }
}

impl AuthStrategy for TwoFactorAuth {
    fn authenticate(&self, credentials: &str) -> bool {
        Self::is_six_digits(credentials) && credentials == self.expected_code
    }
}
