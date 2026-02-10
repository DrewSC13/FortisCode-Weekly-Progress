use audit_proxy_project::{AuditProxy, DecryptionService, RealVault};

#[test]
fn audit_proxy_registers_three_attempts_and_logs_failure_on_invalid() {
    // 1) Instanciar AuditProxy
    let vault = RealVault::new();
    let proxy = AuditProxy::new(vault).with_max_len(8); // pequeño para forzar bloqueo fácil

    // helper cifrado XOR 0xAA (inverso del vault)
    let enc = |s: &str| s.as_bytes().iter().map(|b| b ^ 0xAA).collect::<Vec<u8>>();

    // 2) Realizar 3 intentos de descifrado
    let _ = proxy.decrypt(enc("a"));
    let _ = proxy.decrypt(enc("b"));
    let _ = proxy.decrypt(enc("c"));

    // 3) Verificar exactamente 3 entradas "Intento"
    let logs = proxy.logs();
    let attempts = logs
        .iter()
        .filter(|l| l.starts_with("Intento de descifrado"))
        .count();
    assert_eq!(attempts, 3);

    // 4) Intento con datos inválidos (muy grande), verificar que registró fallo
    let big = vec![0u8; 100];
    let out = proxy.decrypt(big);
    assert_eq!(out, "BLOCKED");

    let logs2 = proxy.logs();
    let attempts2 = logs2
        .iter()
        .filter(|l| l.starts_with("Intento de descifrado"))
        .count();
    assert_eq!(attempts2, 4);

    let failures = logs2
        .iter()
        .filter(|l| l.starts_with("Fallo de descifrado"))
        .count();
    assert_eq!(failures, 1);
}
