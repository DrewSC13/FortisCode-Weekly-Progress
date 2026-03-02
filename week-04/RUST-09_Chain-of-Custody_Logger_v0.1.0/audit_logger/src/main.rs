mod core {
    pub mod chain;
    pub mod signer;
}
mod db {
    pub mod audit_repo;
}

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::{env, net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::core::chain::{audit_data_bytes, compute_current_hash_hex};
use crate::core::signer::SignerWrap;
use crate::db::audit_repo::AuditRepo;

#[derive(Clone)]
struct AppState {
    tx: mpsc::Sender<AuditEvent>,
    repo: Arc<AuditRepo>,
    signer: Arc<SignerWrap>,
}

#[derive(Debug, Clone, Deserialize)]
struct AuditEvent {
    event_type: String,
    payload: Value,
}

#[derive(Debug, Serialize)]
struct QueuedResp {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct VerifyResp {
    ok: bool,
    checked: usize,
    error: Option<String>,
}

/// PostgreSQL guarda TIMESTAMPTZ con precisión de microsegundos.
/// Normalizamos el DateTime antes de hashear/firmar/guardar.
fn normalize_pg_timestamp(ts: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    let micros = ts.timestamp_subsec_micros();
    ts.with_nanosecond(micros * 1000).unwrap()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL env var is required (PostgreSQL)");
    let seed_hex = env::var("AUDIT_ED25519_SEED_HEX")
        .expect("AUDIT_ED25519_SEED_HEX env var is required (Ed25519 32-byte seed hex)");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("failed to connect to Postgres");

    let repo = Arc::new(AuditRepo::new(pool));
    repo.ensure_schema().await.expect("schema init failed");

    let signer = Arc::new(SignerWrap::from_hex_seed(&seed_hex).expect("bad Ed25519 seed"));

    let (tx, mut rx) = mpsc::channel::<AuditEvent>(1024);

    {
        let repo = Arc::clone(&repo);
        let signer = Arc::clone(&signer);

        tokio::spawn(async move {
            while let Some(ev) = rx.recv().await {
                let timestamp = normalize_pg_timestamp(Utc::now());

                let prev_hash = match repo.next_prev_hash().await {
                    Ok(h) => h,
                    Err(e) => {
                        error!("next_prev_hash failed: {e}");
                        continue;
                    }
                };

                let ts = timestamp.to_rfc3339();
                let data = audit_data_bytes(&ev.event_type, &ev.payload, &ts);
                let current_hash = compute_current_hash_hex(&data, &prev_hash);

                let signature = signer.sign_current_hash_b64(&current_hash);

                if let Err(e) = repo
                    .insert_log(
                        &ev.event_type,
                        &ev.payload,
                        timestamp,
                        &prev_hash,
                        &signature,
                    )
                    .await
                {
                    error!("insert_log failed: {e}");
                }
            }
        });
    }

    let state = AppState { tx, repo, signer };

    let app = Router::new()
        .route("/event", post(post_event))
        .route("/verify", get(get_verify))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    info!("audit_logger listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn post_event(State(st): State<AppState>, Json(body): Json<AuditEvent>) -> impl IntoResponse {
    if body.event_type.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "event_type required").into_response();
    }
    match st.tx.try_send(body) {
        Ok(_) => (StatusCode::ACCEPTED, Json(QueuedResp { status: "queued" })).into_response(),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "queue full, try again").into_response(),
    }
}

async fn get_verify(State(st): State<AppState>) -> impl IntoResponse {
    match verify_chain(&st).await {
        Ok(v) => (StatusCode::OK, Json(v)).into_response(),
        Err(e) => {
            let resp = VerifyResp {
                ok: false,
                checked: 0,
                error: Some(e),
            };
            (StatusCode::OK, Json(resp)).into_response()
        }
    }
}

async fn verify_chain(st: &AppState) -> Result<VerifyResp, String> {
    let logs = st
        .repo
        .fetch_all_ordered()
        .await
        .map_err(|e| e.to_string())?;

    if logs.is_empty() {
        return Ok(VerifyResp {
            ok: true,
            checked: 0,
            error: None,
        });
    }

    for i in 0..logs.len() {
        let cur = &logs[i];

        let ts = cur.timestamp.to_rfc3339();
        let data = audit_data_bytes(&cur.event_type, &cur.payload, &ts);
        let current_hash = compute_current_hash_hex(&data, &cur.prev_hash);

        let sig_ok = st
            .signer
            .verify_current_hash_b64(&current_hash, &cur.signature)
            .map_err(|e| e.to_string())?;

        if !sig_ok {
            return Ok(VerifyResp {
                ok: false,
                checked: i + 1,
                error: Some(format!("firma inválida en id={} (posición {})", cur.id, i)),
            });
        }

        if i + 1 < logs.len() {
            let next = &logs[i + 1];
            if next.prev_hash != current_hash {
                return Ok(VerifyResp {
                    ok: false,
                    checked: i + 1,
                    error: Some(format!(
                        "cadena rota entre id={} y id={} (next.prev_hash != current_hash)",
                        cur.id, next.id
                    )),
                });
            }
        }
    }

    Ok(VerifyResp {
        ok: true,
        checked: logs.len(),
        error: None,
    })
}
