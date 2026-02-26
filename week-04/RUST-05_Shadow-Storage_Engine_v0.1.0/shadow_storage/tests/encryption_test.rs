use std::process::Command;

const TEST_KEY_B64: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; // 32 bytes 0x00

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_shadow_storage"))
}

fn run_ok(mut c: Command) -> String {
    let out = c.output().expect("failed to run");
    assert!(
        out.status.success(),
        "expected success\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn run_fail(mut c: Command) -> String {
    let out = c.output().expect("failed to run");
    assert!(
        !out.status.success(),
        "expected failure\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stderr).to_string()
}

#[test]
fn test_unit_uniqueness_same_plaintext_two_encryptions_are_different_due_to_nonce() {
    let mut c1 = bin();
    c1.env("SHADOW_KEY_B64", TEST_KEY_B64)
        .arg("encrypt")
        .arg("same-text");
    let a = run_ok(c1);

    let mut c2 = bin();
    c2.env("SHADOW_KEY_B64", TEST_KEY_B64)
        .arg("encrypt")
        .arg("same-text");
    let b = run_ok(c2);

    assert_ne!(a, b, "debe diferir por nonce único por operación");
}

#[test]
fn test_integrity_bitflip_must_fail_authentication() {
    let mut enc = bin();
    enc.env("SHADOW_KEY_B64", TEST_KEY_B64)
        .arg("encrypt")
        .arg("secret");
    let blob = run_ok(enc);

    let mut flip = bin();
    flip.env("SHADOW_KEY_B64", TEST_KEY_B64)
        .arg("flipbit")
        .arg(&blob);
    let tampered = run_ok(flip);

    let mut dec = bin();
    dec.env("SHADOW_KEY_B64", TEST_KEY_B64)
        .arg("decrypt")
        .arg(&tampered);
    let err = run_fail(dec);

    assert!(
        err.to_lowercase().contains("authentication failed") || err.to_lowercase().contains("auth"),
        "debe fallar por tag mismatch\nerr={err}"
    );
}

#[test]
fn test_visibility_ciphertext_is_illegible_like_bytea_dump() {
    let plaintext = "alice@example.com";

    let mut enc = bin();
    enc.env("SHADOW_KEY_B64", TEST_KEY_B64)
        .arg("encrypt")
        .arg(plaintext);
    let blob_hex = run_ok(enc);

    assert!(
        !blob_hex.contains("alice") && !blob_hex.contains("example"),
        "ciphertext no debe exponer texto en claro"
    );

    let plain_hex = hex::encode(plaintext.as_bytes());
    assert!(
        !blob_hex.contains(&plain_hex),
        "ciphertext no debe contener bytes de plaintext"
    );
}
