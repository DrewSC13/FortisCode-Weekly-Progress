use crate::{Authenticated, User};

/// Métodos exclusivos para usuarios autenticados.
pub fn view_vault_data(user: &User<Authenticated>) -> String {
    // Simulación de datos sensibles protegidos por typestate.
    format!(
        "VAULT_DATA: access granted for {} with token={}",
        user.name(),
        user.token()
    )
}
