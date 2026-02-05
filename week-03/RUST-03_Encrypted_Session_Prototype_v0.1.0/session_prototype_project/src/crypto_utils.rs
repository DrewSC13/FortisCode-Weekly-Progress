use uuid::Uuid;

/// Genera un token de sesion nuevo.
///
/// UUID v4 es aleatorio y practico.
pub fn generate_session_token() -> String {
    Uuid::new_v4().to_string()
}
