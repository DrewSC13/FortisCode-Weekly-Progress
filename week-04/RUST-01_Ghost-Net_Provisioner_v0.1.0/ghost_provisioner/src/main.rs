use anyhow::{anyhow, Context, Result};
use axum::{routing::get, Router};
use std::{env, net::ToSocketAddrs, time::Duration};
use tokio::net::TcpStream;
use tokio_postgres::NoTls;
use tracing::{error, info};

fn env_required(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("Missing env var: {key}"))
}

async fn check_dns() -> Result<()> {
    let db = ("db", 5432)
        .to_socket_addrs()
        .context("DNS resolution failed for db:5432")?
        .next()
        .ok_or_else(|| anyhow!("No DNS results for db"))?;

    let cache = ("cache", 6379)
        .to_socket_addrs()
        .context("DNS resolution failed for cache:6379")?
        .next()
        .ok_or_else(|| anyhow!("No DNS results for cache"))?;

    info!(?db, "DNS ok for db");
    info!(?cache, "DNS ok for cache");
    Ok(())
}

async fn check_tcp(host: &str, port: u16) -> Result<()> {
    let addr = format!("{host}:{port}");
    tokio::time::timeout(Duration::from_secs(2), TcpStream::connect(&addr))
        .await
        .with_context(|| format!("TCP timeout connecting to {addr}"))?
        .with_context(|| format!("TCP connect failed to {addr}"))?;
    Ok(())
}

async fn check_postgres(dsn: &str) -> Result<()> {
    let (client, connection) = tokio_postgres::connect(dsn, NoTls)
        .await
        .context("tokio_postgres::connect failed")?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!(error = %e, "Postgres connection task error");
        }
    });

    let row = client
        .query_one("SELECT 1", &[])
        .await
        .context("SELECT 1 failed")?;
    let v: i32 = row.get(0);
    if v != 1 {
        return Err(anyhow!("Unexpected SELECT 1 result: {v}"));
    }
    Ok(())
}

async fn check_redis(dsn: &str) -> Result<()> {
    let client = redis::Client::open(dsn).context("redis::Client::open failed")?;

    let mut conn = client
        .get_multiplexed_tokio_connection()
        .await
        .context("Redis connect failed")?;

    let pong: String = redis::cmd("PING")
        .query_async(&mut conn)
        .await
        .context("Redis PING failed")?;

    if pong.to_uppercase() != "PONG" {
        return Err(anyhow!("Unexpected Redis PING response: {pong}"));
    }

    let _: () = redis::cmd("SET")
        .arg("gp:health")
        .arg("1")
        .query_async(&mut conn)
        .await
        .context("Redis SET failed")?;

    let got: String = redis::cmd("GET")
        .arg("gp:health")
        .query_async(&mut conn)
        .await
        .context("Redis GET failed")?;

    if got != "1" {
        return Err(anyhow!("Unexpected Redis GET response: {got}"));
    }

    Ok(())
}

async fn do_full_check() -> Result<()> {
    check_dns().await?;

    check_tcp("db", 5432).await.context("TCP check db failed")?;
    check_tcp("cache", 6379)
        .await
        .context("TCP check cache failed")?;

    let pg = env_required("POSTGRES_DSN")?;
    let rd = env_required("REDIS_DSN")?;
    check_postgres(&pg).await.context("Postgres check failed")?;
    check_redis(&rd).await.context("Redis check failed")?;

    info!("self-check OK");
    Ok(())
}

async fn healthz_handler() -> &'static str {
    match do_full_check().await {
        Ok(_) => "ok",
        Err(e) => {
            error!(error = %e, "healthz check failed");
            "fail"
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("serve");

    match mode {
        "serve" => {
            let bind = env::var("APP_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
            let app = Router::new().route("/healthz", get(healthz_handler));

            info!(%bind, "Starting gateway");
            let listener = tokio::net::TcpListener::bind(&bind).await?;
            axum::serve(listener, app).await?;
            Ok(())
        }
        "self-check" => do_full_check().await,
        "dns-check" => check_dns().await,
        _ => Err(anyhow!("Unknown mode. Use: serve | self-check | dns-check")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_required_returns_error_when_missing() {
        let key = "GP_TEST_MISSING_ENV";

        std::env::remove_var(key);

        let err = env_required(key).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("Missing env var"),
            "error should mention missing env var; got: {msg}"
        );
    }

    #[test]
    fn env_required_returns_value_when_present() {
        let key = "GP_TEST_PRESENT_ENV";
        std::env::set_var(key, "ok");

        let v = env_required(key).unwrap();
        assert_eq!(v, "ok");

        std::env::remove_var(key);
    }

    #[tokio::test]
    async fn check_tcp_fails_fast_on_unreachable_port() {
        // Usamos un puerto típicamente cerrado en localhost.
        // No verificamos el texto exacto (depende del SO), solo que falle rápido.
        let res = check_tcp("127.0.0.1", 1).await;
        assert!(res.is_err(), "check_tcp should fail on closed port");
    }

    #[tokio::test]
    async fn do_full_check_fails_outside_docker_without_service_dns() {
        // Fuera de Docker, "db" y "cache" no resuelven: debe fallar.
        let res = do_full_check().await;
        assert!(
            res.is_err(),
            "do_full_check should fail on missing docker DNS/services"
        );
    }
}
