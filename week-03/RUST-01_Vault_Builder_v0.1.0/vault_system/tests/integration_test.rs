use vault_system::{CipherAlgorithm, EncryptionLevel, Vault};

#[test]
fn full_user_flow_builder_chain_build_and_public_api() {
    let vault = Vault::builder()
        .level(EncryptionLevel::High)
        .algorithm(CipherAlgorithm::AES256)
        .timeout(300)
        .key(vec![1, 2, 3, 4, 5, 6, 7, 8])
        .build()
        .unwrap();

    assert!(!vault.get_id().is_empty());
}