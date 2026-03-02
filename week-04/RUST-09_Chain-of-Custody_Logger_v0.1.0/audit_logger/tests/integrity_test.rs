use chrono::{Timelike, Utc};
use serde_json::json;
use sqlx::{PgPool, Row, postgres::PgRow};
use std::env;

use std::sync::OnceLock;
use tokio::sync::Mutex;

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

async fn global_lock() -> tokio::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().await
}

fn normalize_pg_timestamp(ts: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    let micros = ts.timestamp_subsec_micros();
    ts.with_nanosecond(micros * 1000).unwrap()
}

#[path = "../src/core/chain.rs"]
mod chain_impl;

#[path = "../src/core/signer.rs"]
mod signer_impl;

mod core {
    pub mod chain {
        pub use crate::chain_impl::*;
    }
    pub mod signer {
        pub use crate::signer_impl::*;
    }
}

#[path = "../src/db/audit_repo.rs"]
mod audit_repo_impl;

mod db {
    pub mod audit_repo {
        pub use crate::audit_repo_impl::*;
    }
}

use core::chain;
use core::signer::SignerWrap;
use db::audit_repo::AuditRepo;

async fn setup() -> (PgPool, AuditRepo, SignerWrap) {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required for tests");
    let seed_hex = env::var("AUDIT_ED25519_SEED_HEX").expect("AUDIT_ED25519_SEED_HEX required");

    let pool = PgPool::connect(&database_url).await.unwrap();
    let repo = AuditRepo::new(pool.clone());

    repo.ensure_schema().await.unwrap();

    sqlx::query("TRUNCATE TABLE audit_logs RESTART IDENTITY")
        .execute(&pool)
        .await
        .unwrap();

    let signer = SignerWrap::from_hex_seed(&seed_hex).unwrap();
    (pool, repo, signer)
}

async fn insert_one(
    repo: &AuditRepo,
    signer: &SignerWrap,
    event_type: &str,
    payload: serde_json::Value,
) {
    let timestamp = normalize_pg_timestamp(Utc::now());
    let prev_hash = repo.next_prev_hash().await.unwrap();

    let ts = timestamp.to_rfc3339();
    let data = chain::audit_data_bytes(event_type, &payload, &ts);
    let current_hash = chain::compute_current_hash_hex(&data, &prev_hash);

    let signature = signer.sign_current_hash_b64(&current_hash);

    repo.insert_log(event_type, &payload, timestamp, &prev_hash, &signature)
        .await
        .unwrap();
}

async fn verify_all(repo: &AuditRepo, signer: &SignerWrap) -> bool {
    let logs = repo.fetch_all_ordered().await.unwrap();
    if logs.is_empty() {
        return true;
    }

    for i in 0..logs.len() {
        let cur = &logs[i];

        let ts = cur.timestamp.to_rfc3339();
        let data = chain::audit_data_bytes(&cur.event_type, &cur.payload, &ts);
        let current_hash = chain::compute_current_hash_hex(&data, &cur.prev_hash);

        let sig_ok = signer
            .verify_current_hash_b64(&current_hash, &cur.signature)
            .unwrap();
        if !sig_ok {
            return false;
        }

        if i + 1 < logs.len() {
            let next = &logs[i + 1];
            if next.prev_hash != current_hash {
                return false;
            }
        }
    }
    true
}

#[tokio::test]
async fn unit_test_signature_matches_log_content() {
    let _g = global_lock().await;
    let (_pool, _repo, signer) = setup().await;

    let event_type = "LOGIN_FAIL";
    let payload = json!({"user":"alice","ip":"10.0.0.9"});
    let timestamp = normalize_pg_timestamp(Utc::now());
    let prev_hash = chain::genesis_prev_hash();

    let ts = timestamp.to_rfc3339();
    let data = chain::audit_data_bytes(event_type, &payload, &ts);
    let current_hash = chain::compute_current_hash_hex(&data, &prev_hash);

    let sig = signer.sign_current_hash_b64(&current_hash);
    assert!(signer.verify_current_hash_b64(&current_hash, &sig).unwrap());

    let mut altered = current_hash.clone();
    altered.replace_range(0..1, "f");
    assert!(!signer.verify_current_hash_b64(&altered, &sig).unwrap());
}

#[tokio::test]
async fn test_consistency_insert_100_and_prev_hash_chain_ok() {
    let _g = global_lock().await;
    let (_pool, repo, signer) = setup().await;
    for i in 0..100 {
        insert_one(
            &repo,
            &signer,
            "TX",
            json!({"n": i, "action":"credit", "amount": i * 10}),
        )
        .await;
    }

    assert!(verify_all(&repo, &signer).await);
}

#[tokio::test]
async fn test_tampering_detection_manual_db_update_breaks_chain() {
    let _g = global_lock().await;
    let (pool, repo, signer) = setup().await;

    for i in 0..5 {
        insert_one(&repo, &signer, "ALERT", json!({"seq": i, "msg":"ok"})).await;
    }

    // Debe ser válida ANTES del tampering
    assert!(verify_all(&repo, &signer).await);

    let id_row: PgRow = sqlx::query("SELECT id FROM audit_logs ORDER BY id ASC LIMIT 1 OFFSET 2")
        .fetch_one(&pool)
        .await
        .unwrap();
    let tamper_id: i64 = id_row.get::<i64, _>("id");

    sqlx::query("UPDATE audit_logs SET payload = $1 WHERE id = $2")
        .bind(json!({"seq": 999, "msg":"tampered"}))
        .bind(tamper_id)
        .execute(&pool)
        .await
        .unwrap();

    // Debe romperse DESPUÉS del tampering
    assert!(!verify_all(&repo, &signer).await);
}
