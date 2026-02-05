use secure_adapter_project::{LegacyAdapter, SecureData, SecurityLevel};

#[test]
fn secure_data_to_legacy_adapter_encrypt_matches_expected() {
    let adapter = LegacyAdapter::new();

    let input = b"FortisCode".to_vec();
    let secure = SecureData::new(input.clone(), SecurityLevel::High, "demo-secret");

    let encrypted = adapter.encrypt_with_legacy(&secure).unwrap();

    // Verificamos cifrado determinístico (XOR 0xAA) para “expected”.
    let expected: Vec<u8> = input.into_iter().map(|b| b ^ 0xAA).collect();
    assert_eq!(encrypted, expected);

    // Asegura que no devolvemos el input en claro.
    assert_ne!(encrypted, secure.as_slice());
}

#[test]
fn no_logical_leak_in_conversion_path() {
    let adapter = LegacyAdapter::new();

    let secure = SecureData::new(b"super_secret".to_vec(), SecurityLevel::Medium, "token");
    let encrypted = adapter.encrypt_with_legacy(&secure).unwrap();

    // “Fuga lógica”: que el encrypted contenga el original sin transformación.
    assert!(
        !encrypted
            .windows(secure.len())
            .any(|w| w == secure.as_slice())
    );
}
