use crate::{Authenticated, Unauthenticated, User};

/// Contraseña válida simulada (solo para la tarea / testing determinístico).
pub const VALID_PASSWORD: &str = "fortis123";

/// Token esperado tras autenticación exitosa.
pub const TOKEN_OK: &str = "TOKEN_OK";

/// Lógica de transición de estados.
/// Consume `User<Unauthenticated>` y retorna `User<Authenticated>` si el password es correcto.
pub fn authenticate_user(
    user: User<Unauthenticated>,
    password: &str,
) -> Result<User<Authenticated>, String> {
    if password != VALID_PASSWORD {
        return Err("invalid password".to_string());
    }

    Ok(User {
        name: user.name,
        state: Authenticated {
            token: TOKEN_OK.to_string(),
        },
    })
}
