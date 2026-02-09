use auth_strategy_project::{LoginManager, PasswordAuth, TwoFactorAuth};

#[test]
fn login_manager_switches_strategy_at_runtime_password_to_mfa() {
    // 1) Iniciar el LoginManager con estrategia Password.
    let password_strategy = PasswordAuth::from_password("my-secret");
    let mut manager = LoginManager::new(Box::new(password_strategy));

    // 2) Intentar login (passdowrd).
    assert!(manager.login("my-secret"));
    assert!(!manager.login("wrong"));

    // 3) Cambiar estrategia a MFA en caliente.
    let mfa_strategy = TwoFactorAuth::new("123456");
    manager.set_strategy(Box::new(mfa_strategy));

    // 4) Requiere cmportamiento distinto (token 2FA).
    assert!(!manager.login("my-secret"));
    assert!(!manager.login("wrong"));
    assert!(manager.login("123456"));
}
