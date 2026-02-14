use typestate_auth::{TOKEN_OK, User, VALID_PASSWORD};

#[test]
fn full_flow_create_authenticate_and_access_vault_data() {
    // Usuario inicial (no autenticado)
    let user = User::new("Claudio");

    // Autenticar usando la misma clave del sistema (evita desincronización)
    let authed = user
        .authenticate(VALID_PASSWORD)
        .expect("should authenticate");

    // Verificar token correcto tras la transición
    assert_eq!(authed.token(), TOKEN_OK);

    // Acceso permitido solo en estado autenticado
    let data = authed.view_vault_data();
    assert!(data.contains("VAULT_DATA"));
    assert!(data.contains("Claudio"));
    assert!(data.contains(TOKEN_OK));
}
