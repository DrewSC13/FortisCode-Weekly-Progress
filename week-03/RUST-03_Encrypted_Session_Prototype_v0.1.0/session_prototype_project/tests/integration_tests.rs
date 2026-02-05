use session_prototype_project::{Session, SessionPrototype};
use std::collections::HashSet;

#[test]
fn integration_master_and_three_clones_have_unique_tokens_and_same_permissions() {
    let master = Session::new(
        "user-777",
        vec!["read".into(), "write".into(), "admin".into()],
    );

    let c1 = master.spawn_clone();
    let c2 = master.spawn_clone();
    let c3 = master.spawn_clone();

    // Permisos identicos en las cuatro sesiones
    assert_eq!(master.permissions(), c1.permissions());
    assert_eq!(master.permissions(), c2.permissions());
    assert_eq!(master.permissions(), c3.permissions());

    // Tokens distintos (4 unicos)
    let mut tokens = HashSet::new();
    tokens.insert(master.token().to_string());
    tokens.insert(c1.token().to_string());
    tokens.insert(c2.token().to_string());
    tokens.insert(c3.token().to_string());

    assert_eq!(tokens.len(), 4);
}
