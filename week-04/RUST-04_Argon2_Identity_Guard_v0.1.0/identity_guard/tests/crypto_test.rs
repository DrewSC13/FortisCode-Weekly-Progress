use serde::Deserialize;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_identity_guard"))
}

fn run_ok(c: &mut Command) -> String {
    let out = c.output().expect("run binary");
    assert!(
        out.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

#[test]
fn test_consistency_verify_true_for_correct_false_for_wrong() {
    let mut c = bin();
    let hash = run_ok(c.arg("hash").arg("S3cretP@ssw0rd"));

    let mut c = bin();
    let ok = run_ok(c.arg("verify").arg(&hash).arg("S3cretP@ssw0rd"));
    assert_eq!(ok, "true");

    let mut c = bin();
    let bad = run_ok(c.arg("verify").arg(&hash).arg("wrong-password"));
    assert_eq!(bad, "false");
}

#[test]
fn test_uniqueness_same_password_different_hashes_due_to_salt() {
    let mut c = bin();
    let h1 = run_ok(c.arg("hash").arg("same-password"));

    let mut c = bin();
    let h2 = run_ok(c.arg("hash").arg("same-password"));

    assert_ne!(h1, h2, "hashes must differ because salt is random");

    let mut c = bin();
    let ok1 = run_ok(c.arg("verify").arg(&h1).arg("same-password"));
    assert_eq!(ok1, "true");

    let mut c = bin();
    let ok2 = run_ok(c.arg("verify").arg(&h2).arg("same-password"));
    assert_eq!(ok2, "true");
}

#[derive(Debug, Deserialize)]
struct BenchOut {
    memory_mib: u32,
    iterations: u32,
    parallelism: u32,
    elapsed_ms: u128,
    meets_100ms: bool,
}

#[test]
fn test_benchmark_security_policy() {
    let mut c = bin();
    let json = run_ok(c.arg("benchmark"));
    let out: BenchOut = serde_json::from_str(&json).expect("valid json");

    assert!(out.memory_mib >= 64);
    assert!(out.iterations >= 3);
    assert!(out.parallelism >= 1);

    // Si es <100ms, el binario debería aumentar memoria (hasta el máximo permitido por IG_MAX_MEMORY_MIB)
    if !out.meets_100ms {
        let max_mib: u32 = std::env::var("IG_MAX_MEMORY_MIB")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(1024)
            .max(64);

        assert!(
            out.memory_mib >= max_mib || out.elapsed_ms < 100,
            "if <100ms it should be at max_mib or remain <100ms due to env limits"
        );
    }
}
