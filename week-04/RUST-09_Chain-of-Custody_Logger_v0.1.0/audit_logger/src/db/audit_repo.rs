use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};
use thiserror::Error;

use crate::core::chain::{audit_data_bytes, compute_current_hash_hex, genesis_prev_hash};

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuditLog {
    pub id: i64,
    pub event_type: String,
    pub payload: Value,
    pub timestamp: DateTime<Utc>,
    pub prev_hash: String,
    pub signature: String,
}

#[derive(Clone)]
pub struct AuditRepo {
    pool: PgPool,
}

impl AuditRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ensure_schema(&self) -> Result<(), RepoError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS audit_logs (
                id BIGSERIAL PRIMARY KEY,
                event_type TEXT NOT NULL,
                payload JSONB NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                prev_hash TEXT NOT NULL,
                signature TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn fetch_all_ordered(&self) -> Result<Vec<AuditLog>, RepoError> {
        let rows = sqlx::query(
            r#"
            SELECT id, event_type, payload, timestamp, prev_hash, signature
            FROM audit_logs
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            out.push(AuditLog {
                id: r.get::<i64, _>("id"),
                event_type: r.get::<String, _>("event_type"),
                payload: r.get::<Value, _>("payload"),
                timestamp: r.get::<DateTime<Utc>, _>("timestamp"),
                prev_hash: r.get::<String, _>("prev_hash"),
                signature: r.get::<String, _>("signature"),
            });
        }
        Ok(out)
    }

    pub async fn next_prev_hash(&self) -> Result<String, RepoError> {
        let row = sqlx::query(
            r#"
            SELECT event_type, payload, timestamp, prev_hash
            FROM audit_logs
            ORDER BY id DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let event_type = r.get::<String, _>("event_type");
            let payload = r.get::<Value, _>("payload");
            let timestamp = r.get::<DateTime<Utc>, _>("timestamp");
            let prev_hash = r.get::<String, _>("prev_hash");

            let ts = timestamp.to_rfc3339();
            let data = audit_data_bytes(&event_type, &payload, &ts);
            let current_hash = compute_current_hash_hex(&data, &prev_hash);
            Ok(current_hash)
        } else {
            Ok(genesis_prev_hash())
        }
    }

    pub async fn insert_log(
        &self,
        event_type: &str,
        payload: &Value,
        timestamp: DateTime<Utc>,
        prev_hash: &str,
        signature: &str,
    ) -> Result<i64, RepoError> {
        let row = sqlx::query(
            r#"
            INSERT INTO audit_logs (event_type, payload, timestamp, prev_hash, signature)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(event_type)
        .bind(payload)
        .bind(timestamp)
        .bind(prev_hash)
        .bind(signature)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get::<i64, _>("id"))
    }
}
